use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{AppError, AppState};


#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct MosaicConfig {
  pub project: MosaicProjectConfig,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub readme: Option<MosaicReadmeConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website: Option<MosaicWebsiteConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub package: Option<MosaicPackageConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicProjectConfig {
  /// The name of the project.
  pub name: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The description of the project.
  pub description: Option<String>,
  #[serde(default)]
  /// Include the number of stars in the project.
  pub stars: bool,
  #[serde(default = "default_priority")]
  /// The priority of the project.
  /// The higher the number, the higher the priority.
  pub priority: i32,
  #[serde(default)]
  /// Automatically infer the version of the project
  /// based on multiple sources.
  pub version: bool,
  #[serde(default)]
  /// Should the project be ignored.
  pub ignore: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The handle of the project.
  /// By default the handle will be auto generated
  /// based on the repository name.
  pub handle: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// Whether the project is deprecated.
  pub deprecated: Option<Deprecated>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// An alternative project.
  pub alternative: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct Deprecated {
  /// The deprecation message.
  message: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The replacement project.
  replacement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicReadmeConfig {
  /// Include the README of the project.
  enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The path to the README file.
  /// By default the README file will be auto detected.
  path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicPackageConfig {
  /// Include information about the package.
  enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The package type.
  /// By default the package type will be inferred from the repository.
  r#type: Option<PackageType>,
  #[serde(default)]
  /// Include the number of downloads.
  /// If the package type's registry supports it.
  downloads: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The package name.
  /// By default the package name will be inferred from the repository.
  name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
/// The type of package.
pub enum PackageType {
  #[serde(alias="npm", alias="node")]
  /// For NPM Packages
  /// Aliases: npm, node
  NPM,
  #[serde(alias="cargo", alias="crates", alias="rust")]
  /// For Rust Crates
  /// Aliases: cargo, crates, rust
  Cargo,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicWebsiteConfig {
  /// Customize the auto-generated page for the project
  enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The title of the website.
  title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The description of the website.
  description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The URL of the website.
  /// By default the URL will be inferred from the repository.
  url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The keywords to show on the project page.
  keywords: Option<Vec<String>>,
}

fn default_priority() -> i32 {
  10
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResolvedConfig {
  pub content: MosaicConfig,
  pub external: bool,
}

pub async fn resolve_config(
  state: &AppState,
  username: &str,
  repository_name: &str,
) -> Result<ResolvedConfig, AppError> {
  let mut external = false;
  let mut path = ".github/mosaic.toml".to_owned();

  if username != "luxass" {
    external = true;
    path = format!(".github/mosaic/{}/{}.toml", username, repository_name);
  }

  let content = state
    .github
    .get_content_by_path(
      if external { "luxass" } else { username },
      if external { "luxass" } else { repository_name },
      &path,
    )
    .await
    .map_err(|err| {
      tracing::error!("Error getting content by path: {:?}", err);
      err
    })?;

  let decoded_content = content.decoded_content();

  if let Some(content) = decoded_content {
    let resolved_config = toml::from_str::<MosaicConfig>(&content)
      .map_err(|e| AppError::ResolveConfigError(e.to_string()))?;

    return Ok(ResolvedConfig {
      content: resolved_config,
      external,
    });
  }

  Err(AppError::ResolveConfigError("an unhandled error occurred".to_owned()))
}
