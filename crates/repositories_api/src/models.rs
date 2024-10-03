use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{
  openapi::{schema::RefBuilder, Ref},
  ToSchema,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MosaicRepository {
  #[schema(example = "e27319c1-1303-47fd-9bd3-0e00a136200b")]
  /// The id of the repository
  pub id: sqlx::types::Uuid,

  #[schema(example = "R_kgDOJRVGjg")]
  /// The id of the repository on github
  pub github_id: String,

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

  #[schema(example = "2024-10-01T04:56:58.943350Z")]
  /// The owner of the repository
  pub created_at: Option<DateTime<Utc>>,

  #[schema(inline = false, schema_with = mosaic_config_ref, required)]
  /// The config corresponding to the repository
  pub config: Option<sqlx::types::JsonValue>,
}

fn mosaic_config_ref() -> Ref {
  RefBuilder::new()
    .ref_location_from_schema_name("MosaicConfig")
    .build()
}
