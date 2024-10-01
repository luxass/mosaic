use std::collections::HashMap;

use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use github_languages::LANGUAGES;
use mosaic_utils::{ApiErrorResponse, AppState};

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
    (status = OK, description = "An object of languages", body = HashMap<String, String>),
    (status = NOT_FOUND, description = "Not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn handler(
  Path((username, repository_name)): Path<(String, String)>,
  State(state): State<AppState>,
) -> Result<Json<HashMap<String, String>>, ApiErrorResponse> {
  match state
    .github
    .get_languages(&username, &repository_name)
    .await
  {
    Ok(_languages) => {
      let mut languages: HashMap<String, String> = HashMap::new();

      for (language, _) in _languages {
        let found = LANGUAGES.get_by_name(&language);
        if found.is_none() {
          languages.insert(language, "".to_owned());
        } else {
          languages.insert(language, found.unwrap().color.to_owned());
        }
      }

      Ok(Json(languages))
    }
    Err(err) => Err(ApiErrorResponse::from(err)),
  }
}
