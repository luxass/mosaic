use std::collections::HashSet;

use apalis::prelude::Data;
use mosaic_repositories_api::models::MosaicRepository;
use mosaic_utils::{resolve_config, AppError, AppState};

pub async fn update_projects(data: &Data<AppState>) -> Result<(), AppError> {
  tracing::debug!("start updating projects");

  let viewer = data.github.get_user_profile().await?;
  let ignored_projects = fetch_ignored_projects().await?;
  tracing::debug!("{:?}", ignored_projects);

  let nodes = viewer.repositories.nodes.unwrap();
  let repositories = nodes.iter().filter(|repo| {
    if let Some(repo) = repo {
      !repo.is_fork
        && !repo.is_archived
        && !ignored_projects.contains(&repo.name_with_owner)
        && !ignored_projects.contains(&repo.name_with_owner.split("/").nth(1).unwrap().to_string())
    } else {
      false
    }
  });

  tracing::debug!("filtered repositories: {:?}", repositories);

  let mut resolved_configs = Vec::new();
  for repository in repositories {
    if let Some(repository) = repository {
      let parts: Vec<&str> = repository.name_with_owner.split('/').collect();
      let owner = parts.get(0).unwrap();
      let name = parts.get(1).unwrap();
      match resolve_config(data, owner, name).await {
        Ok(resolved_config) => {
          resolved_configs.push((repository, Some(resolved_config)));
        }
        Err(_) => {
          tracing::debug!("no config found for {}", repository.name_with_owner);
          resolved_configs.push((repository, None));
        }
      }
    }
  }

  let repositories_with_config: Vec<_> = resolved_configs
    .into_iter()
    .filter_map(|(repo, config)| {
      if config.is_some() {
        Some((repo, config.unwrap()))
      } else {
        None
      }
    })
    .collect();

  tracing::info!(
    "repositories with config {}",
    repositories_with_config.len()
  );

  let keep_ids = repositories_with_config
    .iter()
    .map(|(repo, _)| &repo.id)
    .collect::<Vec<_>>();

  let keep_ids: Vec<_> = repositories_with_config
    .iter()
    .map(|(repo, _)| repo.id.to_string())
    .collect();
  let placeholders: Vec<String> = (1..=keep_ids.len()).map(|i| format!("${}", i)).collect();

  let joined_ids = keep_ids
    .iter()
    .cloned()
    .map(|id| format!("'{}'", id))
    .collect::<Vec<String>>()
    .join(", ");

  let query = format!(
    "DELETE FROM mosaic_repositories WHERE github_id NOT IN ({});",
    joined_ids
  );

  tracing::info!("query: {}", query);

  let res = sqlx::query(&query).execute(&data.db).await;

  // check if res is an err
  if res.is_err() {
    tracing::info!("error: {:?}", res);
    panic!("hello!")
  }

  tracing::info!("amount {:?}", repositories_with_config);

  let mut repositories_with_errors = Vec::new();

  for (repo, config) in repositories_with_config {
    tracing::debug!("handling {}", repo.name_with_owner);
    let existing_repo = sqlx::query_as!(
      MosaicRepository,
      "SELECT * FROM mosaic_repositories WHERE github_id = $1",
      repo.id,
    )
    .fetch_optional(&data.db)
    .await?;

    if let Some(_existing_repo) = existing_repo {
      let update_result = sqlx::query!("UPDATE mosaic_repositories SET name = $1, name_with_owner = $2, config = $3, last_updated = current_timestamp, description = $4, url = $5 WHERE github_id = $6",
        repo.name, repo.name_with_owner, serde_json::to_value(config)?, repo.description, repo.url, repo.id)
        .execute(&data.db)
        .await;

      if update_result.is_err() {
        tracing::error!(
          "an error occurred while updating a repository: {:?}",
          update_result
        );
        repositories_with_errors.push((repo, update_result));
      }
    } else {
      let insert_result = sqlx::query(
                "INSERT INTO mosaic_repositories (github_id, name_with_owner, description, name, url, config) VALUES ($1, $2, $3, $4, $5, $6)",
            )
                .bind(&repo.id)
                .bind(&repo.name_with_owner)
                .bind(&repo.description)
                .bind(&repo.name)
                .bind(&repo.url)
                .bind(&serde_json::to_value(config)?)
                .execute(&data.db)
                .await;

      if insert_result.is_err() {
        tracing::error!(
          "an error occurred while inserting a new repository: {:?}",
          insert_result
        );
        repositories_with_errors.push((repo, insert_result));
      }
    }
  }

  tracing::info!("repositories with errors: {:?}", repositories_with_errors);

  Ok(())
}

const IGNORE_URL: &str =
  "https://raw.githubusercontent.com/luxass/luxass/main/.github/mosaic/.mosaicignore";

async fn fetch_ignored_projects() -> Result<HashSet<String>, AppError> {
  let response = reqwest::get(IGNORE_URL).await?;

  if response.status().is_success() {
    let ignored_projects = response.text().await?;
    Ok(
      ignored_projects
        .lines()
        .filter(|s| !s.is_empty() || s.starts_with("#"))
        .map(|s| s.to_string())
        .collect(),
    )
  } else {
    Err(AppError::TaskError(
      "failed to fetch ignored projects".to_string(),
    ))
  }
}
