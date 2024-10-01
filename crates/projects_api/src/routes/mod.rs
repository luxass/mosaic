use mosaic_utils::AppState;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::Project;

mod all_projects;
mod get_project;
mod get_project_config;

pub const TAG: &str = "Projects";

#[derive(OpenApi)]
#[openapi(
  components(schemas(
    Project
  )),
  tags(
    (name = TAG, description = "Projects API")
  )
)]
pub struct ProjectsApi;

pub fn routes() -> OpenApiRouter<AppState> {
  OpenApiRouter::with_openapi(ProjectsApi::openapi())
    .routes(routes!(all_projects::handler))
    .routes(routes!(get_project::handler))
    .routes(routes!(get_project_config::handler))
}
