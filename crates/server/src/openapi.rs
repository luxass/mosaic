use mosaic_utils::ApiError;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Mosaic"), components(schemas(ApiError)))]
pub struct ApiDoc;
