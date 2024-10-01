use std::collections::HashMap;

use axum::http::{HeaderMap, HeaderValue};
use chrono::Utc;
use graphql_client::GraphQLQuery;
use profile_query::ProfileQueryViewer;
use reqwest::{
  header::{AUTHORIZATION, USER_AGENT},
  Method,
};
use serde::{Deserialize, Serialize};

use crate::{error::GitHubErrorBody, AppError};
mod models;

#[allow(clippy::upper_case_acronyms)]
type URI = String;

#[allow(clippy::upper_case_acronyms)]
type DateTime = chrono::DateTime<Utc>;

#[derive(GraphQLQuery)]
#[graphql(
  schema_path = "github_schema.graphql",
  query_path = "profile_query.graphql",
  response_derives = "Debug"
)]
pub struct ProfileQuery;

#[derive(Clone, Debug)]
pub struct GitHubClient {
  client: reqwest::Client,
}

impl GitHubClient {
  pub fn new(token: &str) -> Result<Self, AppError> {
    let mut headers = HeaderMap::new();
    headers.insert(
      AUTHORIZATION,
      HeaderValue::from_str(&format!("token {}", token)).unwrap(),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("mosaic"));

    let client = reqwest::Client::builder()
      .default_headers(headers)
      .build()
      .unwrap();

    Ok(Self { client })
  }

  pub async fn get_user_events(
    &self,
    username: &str,
  ) -> Result<Vec<models::events::Event>, AppError> {
    let url = format!(
      "https://api.github.com/users/{}/events?per_page=100&page=1",
      username
    );
    let response = self.client.get(&url).send().await?;

    if response.status().is_success() {
      let events = response.json::<Vec<models::events::Event>>().await?;
      Ok(events)
    } else {
      // TODO: fix this
      let _status = response.status();
      let error_body = response
        .text()
        .await
        .unwrap_or_else(|_| "Unknown error".to_string());

      let github_error = match serde_json::from_str::<GitHubErrorBody>(&error_body) {
        Ok(parsed_error) => parsed_error,
        Err(_) => GitHubErrorBody {
          documentation_url: None,
          errors: None,
          message: error_body.clone(),
        },
      };

      Err(AppError::GitHubError(GitHubErrorBody {
        documentation_url: github_error.documentation_url,
        errors: github_error.errors,
        message: github_error.message,
      }))
    }
  }

  pub async fn get_user_profile(&self) -> Result<ProfileQueryViewer, AppError> {
    let response = self
      .client
      .post("https://api.github.com/graphql")
      .json(&ProfileQuery::build_query(profile_query::Variables {}))
      .send()
      .await?;

    if response.status().is_success() {
      let profile = response
        .json::<graphql_client::Response<profile_query::ResponseData>>()
        .await?;
      if let Some(errors) = profile.errors {
        return Err(AppError::GitHubError(GitHubErrorBody {
          documentation_url: None,
          errors: Some(errors.into_iter().map(|e| serde_json::json!(e)).collect()),
          message: "Failed to fetch user profile".to_string(),
        }));
      }

      Ok(profile.data.unwrap().viewer)
    } else {
      Err(AppError::GitHubError(GitHubErrorBody {
        documentation_url: None,
        errors: None,
        message: "Failed to fetch user profile".to_string(),
      }))
    }
  }

  pub async fn get_languages(
    &self,
    username: &str,
    repository_name: &str,
  ) -> Result<HashMap<String, i32>, AppError> {
    let url = format!(
      "https://api.github.com/repos/{}/{}/languages",
      username, repository_name
    );
    let response = self
      .client
      .request(Method::GET, &url)
      .header("Accept", "application/vnd.github+json")
      .send()
      .await?;

    if !response.status().is_success() {
      let status = response.status();
      let error_body = response
        .text()
        .await
        .unwrap_or_else(|_| "Unknown error".to_string());

      Err(AppError::GitHubError(GitHubErrorBody {
        documentation_url: None,
        errors: None,
        message: format!(
          "GitHub API error: status = {}, message = {}",
          status, error_body
        ),
      }))
    } else {
      let languages = response.json::<HashMap<String, i32>>().await?;
      Ok(languages)
    }
  }

  pub async fn get_content_by_path(
    &self,
    username: &str,
    repository_name: &str,
    path: &str,
  ) -> Result<GitHubContentObject, AppError> {
    let url = format!(
      "https://api.github.com/repos/{}/{}/contents/{}",
      username, repository_name, path
    );

    tracing::debug!("requesting github content from {}", url);

    let response = self
      .client
      .request(Method::GET, &url)
      .header("Accept", "application/vnd.github+json")
      .send()
      .await?;

    if !response.status().is_success() {
      let status = response.status();
      let error_body = response
        .text()
        .await
        .unwrap_or_else(|_| "Unknown error".to_string());

      Err(AppError::GitHubError(GitHubErrorBody {
        documentation_url: None,
        errors: None,
        message: format!(
          "GitHub API error: status = {}, message = {}",
          status, error_body
        ),
      }))
    } else {
      let content = response
        .json::<GitHubContentObject>()
        .await
        .map_err(|err| {
          tracing::error!("Error parsing GitHub content response: {:?}", err);
          AppError::ParseConfigError(err.to_string())
        })?;

      Ok(content)
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubContentObject {
  pub name: String,
  pub path: String,
  pub sha: String,
  pub size: i32,
  pub url: String,
  pub html_url: String,
  pub git_url: String,
  pub download_url: String,
  pub r#type: String,
  pub content: Option<String>,
  pub encoding: String,
  pub _links: HashMap<String, String>,
}

impl GitHubContentObject {
  pub fn decoded_content(&self) -> Option<String> {
    use base64::Engine;
    self.content.as_ref().map(|c| {
      let mut content = c.as_bytes().to_owned();
      content.retain(|b| !b" \n\t\r\x0b\x0c".contains(b));
      let c = base64::prelude::BASE64_STANDARD.decode(content).unwrap();
      String::from_utf8_lossy(&c).into_owned()
    })
  }
}
