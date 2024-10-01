use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use mosaic_utils::{ApiErrorResponse, AppState, ResolvedConfig};

use crate::TAG;

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/mosaic/{username}/{repository_name}",
  params(
    ("username", Path, description = "GitHub Username"),
    ("repository_name", Path, description = "GitHub Repository Name"),
  ),
  responses(
    (status = OK, description = "A resolved config", body = ResolvedConfig),
    (status = NOT_FOUND, description = "Not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn handler(
  Path((username, repository_name)): Path<(String, String)>,
  State(state): State<AppState>,
) -> Result<Json<ResolvedConfig>, ApiErrorResponse> {
  let resolved_config = mosaic_utils::resolve_config(&state, &username, &repository_name)
    .await
    .map_err(|err| {
      tracing::error!("Error resolving config: {:?}", err);
      ApiErrorResponse::from(err)
    })?;

  Ok(Json(resolved_config))
}
