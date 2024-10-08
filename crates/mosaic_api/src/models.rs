use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResolvedProject {
  pub name: String,
  pub description: Option<String>,
  pub ignore: bool,
  pub priority: u8,

  pub handle: Option<String>,

  #[serde(skip_serializing_if = "Option::is_none")]
  pub stars: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<String>,
}
