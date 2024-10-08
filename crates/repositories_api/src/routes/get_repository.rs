use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};
use uuid::Uuid;

use crate::{models::MosaicRepository, TAG};

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/repositories/{mosaic_repository_id}",
  params(
    ("mosaic_repository_id", Path, description = "Id of the mosaic repository"),
  ),
  responses(
    (status = OK, description = "The Project", body = MosaicRepository),
    (status = NOT_FOUND, description = "Project not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
  )
)]
#[debug_handler]
pub async fn handler(
  Path(mosaic_repository_id): Path<Uuid>,
  State(state): State<AppState>,
) -> Result<Json<MosaicRepository>, ApiErrorResponse> {
  match sqlx::query_as!(
    MosaicRepository,
    "SELECT * FROM mosaic_repositories WHERE id = $1",
    mosaic_repository_id
  )
  .fetch_one(&state.db)
  .await
  {
    Ok(repository) => Ok(Json(repository)),
    Err(err) => {
      tracing::error!("Failed to fetch projects: {:?}", err);
      Err(ApiErrorResponse::from(AppError::SqlxError(err)))
    }
  }
}
