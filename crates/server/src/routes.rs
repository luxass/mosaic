use axum::response::IntoResponse;
use axum::{routing::get, Json, Router};
use mosaic_utils::{get_json_schema, AppState};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::openapi::ApiDoc;

pub fn routes() -> Router<AppState> {
  let (api_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
    .merge(mosaic_repositories_api::routes())
    .merge(mosaic_api::routes())
    .split_for_parts();

  let openapi_json = api.clone();

  api_router
    .route("/openapi.json", get(|| async move { Json(openapi_json) }))
    .route("/json-schema", get(json_schema_handler))
    .route("/json-schema.json", get(json_schema_handler))
    .merge(Scalar::with_url("/scalar", api))
}

async fn json_schema_handler() -> impl IntoResponse {
  let schema = get_json_schema().await;
  Json(schema)
}
