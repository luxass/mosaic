use apalis::prelude::Data;
use mosaic_utils::{AppError, AppState};

pub async fn update_projects(data: &Data<AppState>) -> Result<(), AppError> {
  tracing::debug!("start updating projects");

  let viewer = data.github.get_user_profile().await?;

  let ignored = fetch_ignored_projects().await?;

  tracing::debug!("{:?}", ignored);

  // fetch all ignored mosaic projects
  // from https://raw.githubusercontent.com/luxass/luxass/main/.github/mosaic/.mosaicignore

  // const repositories = viewer.repositories.nodes.filter((repo): repo is NonNullable<Repository> => {
  //     return (
  //       !!repo
  //       && !repo.isFork
  //       && !repo.isPrivate
  //       && !repo.isArchived
  //       && !ignore.includes(repo.nameWithOwner)
  //       && !ignore.includes(repo.nameWithOwner.split("/")[1])
  //     );
  //   });

  let nodes = viewer.repositories.nodes.unwrap();
  let repositories = nodes.iter().filter(|repo| {
    if let Some(repo) = repo {
      !repo.is_fork
        && !repo.is_archived
        && !ignored.contains(&repo.name_with_owner)
        && !ignored.contains(&repo.name_with_owner.split("/").nth(1).unwrap().to_string())
    } else {
      false
    }
  });

  tracing::debug!("filtered repositories: {:?}", repositories);

  // resolve `mosaic.toml` from each repository

  // remove all repositories that doesn't have config anymore.

  // check if repo already exists in db
  // if not, insert it,
  // if yes, update it.

  Ok(())
}

async fn fetch_ignored_projects() -> Result<Vec<String>, AppError> {
  let response = reqwest::get(
    "https://raw.githubusercontent.com/luxass/luxass/main/.github/mosaic/.mosaicignore",
  )
  .await?;

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
      "Failed to fetch ignored projects".to_string(),
    ))
  }
}
