use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MosaicEvent {
  pub id: String,
  pub event_type: String,
  pub created_at: DateTime<Utc>,
  pub repo: String,
}
