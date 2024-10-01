use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use mosaic_utils::MosaicConfig;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MosaicRepository {
  #[schema(example = "e27319c1-1303-47fd-9bd3-0e00a136200b")]
  /// The id of the repository
  pub id: sqlx::types::Uuid,

  #[schema(example = "692848155")]
  /// The id of the repository on github
  pub github_id: i64,

  #[schema(example = "luxass/github-schema")]
  /// The name of the repository with the owner
  pub name_with_owner: String,

  #[schema(example = "GitHub's GraphQL schema")]
  /// The description of the repository
  pub description: Option<String>,

  #[schema(example = "github-schema")]
  /// The name of the repository
  pub name: String,

  #[schema(example = "https://github.com/luxass/github-schema")]
  /// The owner of the repository
  pub url: String,

  #[schema(example = "2024-10-01T04:56:58.943350Z")]
  /// The owner of the repository
  pub last_updated: Option<DateTime<Utc>>,

  #[schema(value_type = MosaicConfig, example = json!({
    "project": {
      "description": "The website you're currently viewing.",
      "ignore": false,
      "name": "luxass.dev",
      "priority": 10,
      "stars": false,
      "version": false
    },
    "website": {
      "enabled": true
    }
  }))]
  /// The owner of the repository
  pub config: Option<sqlx::types::JsonValue>,
}
