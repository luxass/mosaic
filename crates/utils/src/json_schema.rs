use axum::response::IntoResponse;
use schemars::gen::SchemaSettings;

use crate::MosaicConfig;

pub async fn get_json_schema() -> impl IntoResponse {
  let settings = SchemaSettings::draft07().with(|s| {
    s.option_nullable = true;
    s.option_add_null_type = false;
  });
  let generator = settings.into_generator();
  let schema = generator.into_root_schema_for::<MosaicConfig>();

  serde_json::to_string_pretty(&schema).unwrap()
}
