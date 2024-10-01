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

  // TODO: handle this error

  match serde_json::to_string_pretty(&schema) {
    Ok(json_schema) => json_schema,
    Err(err) => {
      tracing::error!("failed to serialize json schema: {}", err);
      "{}".to_string()
    }
  }
}
