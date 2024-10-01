use axum::{debug_handler, extract::State, Json};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};

use crate::{models::Project, TAG};

#[utoipa::path(
  get,
  path = "/api/v1/projects",
  tag = TAG,
  responses(
    (status = OK, description = "List of Projects", body = Vec<Project>),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn handler(
  State(state): State<AppState>,
) -> Result<Json<Vec<Project>>, ApiErrorResponse> {
  match sqlx::query_as!(Project, "SELECT * FROM projects")
    .fetch_all(&state.db)
    .await
  {
    Ok(projects) => Ok(Json(projects)),
    Err(err) => {
      tracing::error!("Failed to fetch projects: {:?}", err);
      Err(ApiErrorResponse::from(AppError::SqlxError(err)))
    }
  }
}
