use std::collections::HashMap;

use axum::http::StatusCode;
use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use github_languages::LANGUAGES;
use mosaic_utils::{ApiErrorResponse, AppError, AppState};

use crate::TAG;

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/mosaic/{username}/{repository_name}/languages",
  params(
    ("username", Path, description = "GitHub Username"),
    ("repository_name", Path, description = "GitHub Repository Name"),
  ),
  responses(
    (status = OK, description = "An object of languages", body = HashMap<String, String>, example = json!({
      "Typescript": "#3178c6",
    })),
    (status = NOT_FOUND, description = "Not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn handler(
  Path((username, repository_name)): Path<(String, String)>,
  State(state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, ApiErrorResponse> {
  let github_languages = state
    .github
    .repos(&username, &repository_name)
    .list_languages()
    .await
    .map_err(|err| {
      tracing::error!(
        "an error occurred while trying to fetch languages used: {:?}",
        err
      );

      if let octocrab::Error::GitHub { source, .. } = err {
        if source.status_code == StatusCode::NOT_FOUND {
          return ApiErrorResponse::from(AppError::NotFound);
        }
      }

      ApiErrorResponse::from(AppError::Unknown)
    })?;

  let mut languages: HashMap<String, String> = HashMap::new();

  for (language, _) in github_languages {
    let found = LANGUAGES.get_by_name(&language);
    if let Some(found) = found {
      languages.insert(language, found.color.to_owned());
    } else {
      languages.insert(language, "".to_owned());
    }
  }

  Ok(Json(languages))
}
