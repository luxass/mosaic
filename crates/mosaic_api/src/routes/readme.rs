use std::{fmt, str::FromStr};

use axum::{
  debug_handler,
  extract::{Path, Query, State},
};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};
use serde::{de, Deserialize, Deserializer};
use utoipa::IntoParams;

use crate::TAG;

#[derive(Debug, Deserialize, IntoParams)]
pub struct TransformQuery {
  #[serde(default, deserialize_with = "empty_string_as_true")]
  #[param(example = "false", default = "false")]
  /// Whether to transform the README content.
  /// If true, the content will be transformed
  /// using a set of predefined rules.
  pub transform: Option<bool>,
}

fn empty_string_as_true<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: FromStr,
  T::Err: fmt::Display,
{
  let opt = Option::<String>::deserialize(de)?;
  // if opt is None or an empty string, return true
  match opt.as_deref() {
    None | Some("") => FromStr::from_str("true")
      .map_err(de::Error::custom)
      .map(Some),
    Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
  }
}

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/mosaic/{username}/{repository_name}/readme",
  params(
    ("username", Path, description = "GitHub Username"),
    ("repository_name", Path, description = "GitHub Repository Name"),
    TransformQuery,
  ),
  responses(
    (status = OK, description = "The repository README", body = String),
    (status = NOT_FOUND, description = "Not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn get_root_readme_handler(
  Path((username, repository_name)): Path<(String, String)>,
  State(state): State<AppState>,
  query: Query<TransformQuery>,
) -> Result<String, ApiErrorResponse> {
  let content = state
    .github
    .repos(&username, &repository_name)
    .get_content()
    .path("README.md")
    .send()
    .await
    .map_err(|err| {
      tracing::error!("an error occurred while trying to get file: {:?}", err);
      ApiErrorResponse::from(AppError::from(err))
    })?;

  // TODO: improve this
  // check if any of the items is named README.md
  let readme = match content.items.iter().any(|item| item.name == "README.md") {
    true => content
      .items
      .iter()
      .find(|item| item.name == "README.md")
      .unwrap(),
    false => {
      return Err(ApiErrorResponse::from(AppError::NotFound));
    }
  };

  let decoded_content = match readme.decoded_content() {
    None => {
      tracing::error!("could not decode content");
      return Err(ApiErrorResponse::from(AppError::Unknown));
    }
    Some(content) => content,
  };

  let should_transform = query.transform.unwrap_or(false);
  tracing::debug!("should_transform: {}", should_transform);

  if should_transform {
    return Ok("# Hello, world!".to_string());
  }

  Ok(decoded_content)
}

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/mosaic/{username}/{repository_name}/readme/{path}",
  params(
    ("username", Path, description = "GitHub Username"),
    ("repository_name", Path, description = "GitHub Repository Name"),
    ("path", Path, description = "Path to the README file"),
    TransformQuery,
  ),
  responses(
    (status = OK, description = "The repository README", body = String),
    (status = NOT_FOUND, description = "Not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn get_readme_by_path_handler(
  Path((username, repository_name, path)): Path<(String, String, String)>,
  State(state): State<AppState>,
  query: Query<TransformQuery>,
) -> Result<String, ApiErrorResponse> {
  let content = state
    .github
    .repos(&username, &repository_name)
    .get_content()
    .path(&path)
    .send()
    .await
    .map_err(|err| {
      tracing::error!("an error occurred while trying to get file: {:?}", err);
      ApiErrorResponse::from(AppError::from(err))
    })?;

  // TODO: improve this
  // check if any of the items is named README.md
  let readme = match content.items.iter().any(|item| item.name == "README.md") {
    true => content
      .items
      .iter()
      .find(|item| item.name == "README.md")
      .unwrap(),
    false => {
      return Err(ApiErrorResponse::from(AppError::NotFound));
    }
  };

  let decoded_content = match readme.decoded_content() {
    None => {
      tracing::error!("could not decode content");
      return Err(ApiErrorResponse::from(AppError::Unknown));
    }
    Some(content) => content,
  };

  let should_transform = query.transform.unwrap_or(false);
  tracing::debug!("should_transform: {}", should_transform);

  if should_transform {
    return Ok("# Hello, world!".to_string());
  }

  Ok(decoded_content)
}
