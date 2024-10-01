use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Project {
  pub id: sqlx::types::Uuid,
  pub github_id: i64,
  pub name_with_owner: String,
  pub description: Option<String>,
  pub name: String,
  pub url: String,
  pub last_updated: Option<DateTime<Utc>>,
  // pub config: Option<Json<MosaicConfig>>,
  pub config: Option<sqlx::types::JsonValue>
}
