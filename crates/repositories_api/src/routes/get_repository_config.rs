use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use mosaic_utils::{ApiErrorResponse, AppError, AppState, MosaicConfig};
use uuid::Uuid;

use crate::TAG;

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/repositories/{mosaic_repository_id}/config",
  params(
    ("mosaic_repository_id", Path, description = "Id of the project"),
  ),
  responses(
    (status = OK, description = "The Project Config for the given project", body = MosaicConfig),
    (status = NOT_FOUND, description = "Project not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
  )
)]
#[debug_handler]
pub async fn handler(
  Path(mosaic_repository_id): Path<Uuid>,
  State(state): State<AppState>,
) -> Result<Json<MosaicConfig>, ApiErrorResponse> {
  match sqlx::query_scalar!("SELECT config FROM mosaic_repositories WHERE id = $1", mosaic_repository_id)
    .fetch_one(&state.db)
    .await
  {
    Ok(raw_config) => {
      let config: MosaicConfig = serde_json::from_value(raw_config).map_err(|err| {
        tracing::error!("Failed to parse config: {:?}", err);
        ApiErrorResponse::from(AppError::SerdeJsonError(err))
      })?;

      Ok(Json(config))
    }
    Err(err) => {
      if let sqlx::Error::RowNotFound = err {
        return Err(ApiErrorResponse::from(AppError::NotFound));
      }

      tracing::error!("Failed to fetch projects: {:?}", err);
      Err(ApiErrorResponse::from(AppError::SqlxError(err)))
    }
  }
}
