use std::collections::HashMap;

use reqwest::StatusCode;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{AppError, AppState};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct BaseMosaicConfig {
  pub project: MosaicProjectConfig,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub readme: Option<MosaicReadmeConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub website: Option<MosaicWebsiteConfig>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub package: Option<MosaicPackageConfig>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
#[schema(example = json!({
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
pub struct MosaicConfig {
  #[serde(flatten)]
  pub base_config: BaseMosaicConfig,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub workspace: Option<MosaicWorkspaceConfig>,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicWorkspaceConfig {
  /// Include the workspace of the project.
  pub enabled: bool,

  #[serde(skip_serializing_if = "Option::is_none")]
  /// The ignored projects in this workspace.
  pub ignores: Option<Vec<String>>,

  #[serde(skip_serializing_if = "Option::is_none")]
  /// Overrides
  pub overrides: Option<HashMap<String, BaseMosaicConfig>>,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicProjectConfig {
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The name of the project.
  /// By default the name will be inferred from the repository name.
  pub name: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The description of the project.
  pub description: Option<String>,
  #[serde(default)]
  /// Include the number of stars in the project.
  pub stars: bool,
  #[serde(default = "default_priority")]
  /// The priority of the project.
  /// The higher the number, the higher the priority.
  pub priority: u8,
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
  pub enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The package type.
  /// By default the package type will be inferred from the repository.
  pub r#type: Option<PackageType>,
  #[serde(default)]
  /// Include the number of downloads.
  /// If the package type's registry supports it.
  pub downloads: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The package name.
  /// By default the package name will be inferred from the repository.
  pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ToSchema)]
/// The type of package.
pub enum PackageType {
  #[serde(alias = "npm", alias = "node")]
  /// For NPM Packages
  /// Aliases: npm, node
  NPM,
  #[serde(alias = "cargo", alias = "crates", alias = "rust")]
  /// For Rust Crates
  /// Aliases: cargo, crates, rust
  Cargo,
}

#[derive(Debug, Serialize, Deserialize, Default, JsonSchema, ToSchema)]
pub struct MosaicWebsiteConfig {
  /// Customize the auto-generated page for the project
  pub enabled: bool,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The title of the website.
  pub title: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The description of the website.
  pub description: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The URL of the website.
  /// By default the URL will be inferred from the repository.
  pub url: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  /// The keywords to show on the project page.
  pub keywords: Option<Vec<String>>,
}

fn default_priority() -> u8 {
  10
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ResolvedConfig {
  pub content: MosaicConfig,
  /// Whether the config is externally sourced.
  /// If the config is externally sourced, it is loaded from `luxass/luxass`.
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

  let content_items = state
    .github
    .repos(
      if external { "luxass" } else { username },
      if external { "luxass" } else { repository_name },
    )
    .get_content()
    .path(&path)
    .send()
    .await
    .map_err(|err| {
      tracing::error!("an error occurred while trying to get content: {:?}", path);
      if let octocrab::Error::GitHub { source, .. } = &err {
        if source.status_code == StatusCode::NOT_FOUND {
          tracing::error!("the path was not found");
          return AppError::NotFound;
        }
      }

      tracing::error!("error getting content: {}", err);

      AppError::from(err)
    })?;

  let content = match content_items.items.first() {
    None => {
      return Err(AppError::ResolveConfigError(
        "mosaic config not found".to_string(),
      ));
    }
    Some(config) => config,
  };

  match content.decoded_content() {
    None => Err(AppError::ResolveConfigError(
      "could not decode mosaic config".to_string(),
    )),
    Some(config) => {
      let resolved_config = toml::from_str::<MosaicConfig>(&config)
        .map_err(|e| AppError::ResolveConfigError(e.to_string()))?;

      Ok(ResolvedConfig {
        content: resolved_config,
        external,
      })
    }
  }
}
