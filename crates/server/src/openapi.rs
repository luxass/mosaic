use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  info(title = "Mosaic"),
  components(schemas(mosaic_utils::ApiError, mosaic_utils::WrappedStatusCode))
)]
pub struct ApiDoc;
