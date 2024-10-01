use mosaic_utils::{
  AppState, MosaicConfig, MosaicPackageConfig, MosaicProjectConfig, MosaicReadmeConfig,
  MosaicWebsiteConfig, PackageType, ResolvedConfig,
};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod config;
mod languages;
mod readme;
mod resolve;

pub const TAG: &str = "Mosaic";

#[derive(OpenApi)]
#[openapi(
  components(schemas(
    ResolvedConfig,
    MosaicConfig,
    MosaicWebsiteConfig,
    MosaicPackageConfig,
    MosaicProjectConfig,
    MosaicReadmeConfig,
    PackageType
  )),
  tags(
    (name = TAG, description = "Mosaic API")
  )
)]
pub struct MosaicApi;

pub fn routes() -> OpenApiRouter<AppState> {
  OpenApiRouter::with_openapi(MosaicApi::openapi())
    .routes(routes!(languages::handler))
    .routes(routes!(config::handler))
    .routes(routes!(readme::get_root_readme_handler))
    .routes(routes!(readme::get_readme_by_path_handler))
    .routes(routes!(resolve::handler))
}
