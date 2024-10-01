use axum::{debug_handler, extract::State, Json};
use mosaic_utils::{ApiErrorResponse, AppState};

use crate::{models::MosaicEvent, TAG};

#[utoipa::path(
  get,
  path = "/api/v1/events",
  tag = TAG,
  responses(
    (status = OK, description = "List of GitHub Events", body = Vec<MosaicEvent>),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ApiError)
  )
)]
#[debug_handler]
pub async fn get_events(State(state): State<AppState>) -> Result<Json<Vec<MosaicEvent>>, ApiErrorResponse> {
  match state.github.get_user_events("luxass").await {
    Ok(events) => {
      let mapped_events = events
        .into_iter()
        .map(|event| MosaicEvent {
          id: event.id,
          event_type: event.r#type,
          created_at: event.created_at,
          repo: event.repo.name,
        })
        .collect();

      Ok(Json(mapped_events))
    }
    Err(e) => {
      tracing::error!("failed to fetch github events: {:?}", e);
      Err(ApiErrorResponse::from(e))
    }
  }
}
