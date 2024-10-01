use mosaic_utils::AppState;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::MosaicRepository;

mod all_repositories;
mod get_repository;
mod get_repository_config;

pub const TAG: &str = "Repositories";

#[derive(OpenApi)]
#[openapi(
  components(schemas(
    MosaicRepository
  )),
  tags(
    (name = TAG, description = "Repoitory API")
  )
)]
pub struct ProjectsApi;

pub fn routes() -> OpenApiRouter<AppState> {
  OpenApiRouter::with_openapi(ProjectsApi::openapi())
    .routes(routes!(all_repositories::handler))
    .routes(routes!(get_repository::handler))
    .routes(routes!(get_repository_config::handler))
}
