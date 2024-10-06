use schemars::gen::SchemaSettings;
use schemars::schema::RootSchema;

use crate::MosaicConfig;

pub async fn get_json_schema() -> RootSchema {
  let settings = SchemaSettings::draft07().with(|s| {
    s.option_nullable = true;
    s.option_add_null_type = false;
  });
  let generator = settings.into_generator();
  generator.into_root_schema_for::<MosaicConfig>()
}
