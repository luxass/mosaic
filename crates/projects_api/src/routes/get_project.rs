use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};
use uuid::Uuid;

use crate::{models::Project, TAG};

#[utoipa::path(
  get,
  tag = TAG,
  path = "/api/v1/projects/{project_id}",
  params(
    ("project_id", Path, description = "Id of the project"),
  ),
  responses(
    (status = OK, description = "The Project", body = Project),
    (status = NOT_FOUND, description = "Project not found", body = ApiError),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError),
  )
)]
#[debug_handler]
pub async fn handler(
  Path(project_id): Path<Uuid>,
  State(state): State<AppState>,
) -> Result<Json<Project>, ApiErrorResponse> {
  match sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = $1", project_id)
    .fetch_one(&state.db)
    .await
  {
    Ok(project) => Ok(Json(project)),
    Err(err) => {
      tracing::error!("Failed to fetch projects: {:?}", err);
      Err(ApiErrorResponse::from(AppError::SqlxError(err)))
    }
  }
}
