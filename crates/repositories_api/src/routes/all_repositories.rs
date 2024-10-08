use axum::{debug_handler, extract::State, Json};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};

use crate::{models::MosaicRepository, TAG};

#[utoipa::path(
  get,
  path = "/api/v1/repositories",
  tag = TAG,
  responses(
    (status = OK, description = "List of Projects", body = Vec<MosaicRepository>),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn handler(
  State(state): State<AppState>,
) -> Result<Json<Vec<MosaicRepository>>, ApiErrorResponse> {
  match sqlx::query_as!(MosaicRepository, "SELECT * FROM mosaic_repositories")
    .fetch_all(&state.db)
    .await
  {
    Ok(repositories) => Ok(Json(repositories)),
    Err(err) => {
      tracing::error!("Failed to fetch repositories: {:?}", err);
      Err(ApiErrorResponse::from(AppError::SqlxError(err)))
    }
  }
}
