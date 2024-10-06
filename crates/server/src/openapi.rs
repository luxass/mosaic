use utoipa::OpenApi;
use mosaic_utils::ApiError;

#[derive(OpenApi)]
#[openapi(
  info(title = "Mosaic"),
  components(schemas(
    ApiError
  ))
)]
pub struct ApiDoc;
