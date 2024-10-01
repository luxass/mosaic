use mosaic_utils::AppState;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::models::MosaicEvent;

mod all;
mod pull_requests;
mod push;

pub const TAG: &str = "Events";

#[derive(OpenApi)]
#[openapi(
  components(schemas(MosaicEvent)),
  tags(
    (name = TAG, description = "Events API")
  )
)]
pub struct EventsApi;

pub fn routes() -> OpenApiRouter<AppState> {
  OpenApiRouter::with_openapi(EventsApi::openapi())
    .routes(routes!(push::get_push_events))
    .routes(routes!(pull_requests::get_pull_request_events))
    .routes(routes!(all::get_events))
}
