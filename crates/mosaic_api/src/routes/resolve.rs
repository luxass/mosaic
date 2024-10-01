use axum::{
  debug_handler,
  extract::{Path, State},
  Json,
};
use mosaic_utils::{resolve_config, ApiErrorResponse, AppEnv, AppError, AppState, ResolvedConfig};

use crate::{models::ResolvedProject, TAG};

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
) -> Result<Json<Vec<ResolvedProject>>, ApiErrorResponse> {
  let resolved_config = mosaic_utils::resolve_config(&state, &username, &repository_name)
    .await
    .map_err(|err| {
      tracing::error!("Error resolving config: {:?}", err);
      ApiErrorResponse::from(err)
    })?;

  let repository = state
    .github
    .get_repository(&username, &repository_name)
    .await
    .map_err(|err| {
      tracing::error!("Error getting repository: {:?}", err);
      ApiErrorResponse::from(err)
    })?;

  let mosaic_config = resolved_config.content;

  if mosaic_config.base_config.project.ignore {
    return Err(ApiErrorResponse::from(AppError::IgnoredProject));
  }

  let mut resolved_projects: Vec<ResolvedProject> = Vec::new();

  if let Some(_workspace_config) = &mosaic_config.workspace {
    return Err(ApiErrorResponse::from(AppError::Unknown));
  }


  let project = ResolvedProject {
    name: if let Some(name) = &mosaic_config.base_config.project.name {
      name.clone()
    } else {
      repository_name.clone()
    },
    ignore: mosaic_config.base_config.project.ignore,
    priority: mosaic_config.base_config.project.priority,
    description: mosaic_config.base_config.project.description.clone(),
    handle: mosaic_config.base_config.project.handle.clone(),
    version: Some("0.0.1".to_string()),
    stars: Some(10),
  };

  resolved_projects.push(project);

  Ok(Json(resolved_projects))
}
