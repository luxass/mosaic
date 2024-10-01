use axum::{debug_handler, extract::{Path, State}, Json};
use mosaic_utils::{ApiErrorResponse, AppError, AppState};
use uuid::Uuid;

use crate::{models::Project, TAG};



#[utoipa::path(
  get,
  path = "/api/v1/projects/{project_id}",
  tag = TAG,
  responses(
    (status = OK, description = "List of Projects"),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error")
  )
)]
#[debug_handler]
pub async fn handler(
  Path(project_id): Path<Uuid>,
  State(state): State<AppState>
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
