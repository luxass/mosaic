use std::env;

use crate::{AppError, GitHubClient};

#[derive(Clone, Debug)]
pub struct AppState {
  pub env: AppEnv,
  pub github: GitHubClient,
  pub db: sqlx::PgPool,
}

#[derive(Clone, Debug)]
pub struct AppEnv {
  pub database_url: String,
  pub mosaic_listen_host: String,
  pub mosaic_listen_port: String,
  pub github_token: String,
}

impl AppEnv {
  pub fn init() -> Result<Self, AppError> {
    Ok(Self {
      database_url: env::var("DATABASE_URL")
        .unwrap_or_else(|_| panic!("DATABASE_URL is not set, please refer to the documentation.")),
      mosaic_listen_host: env::var("MOSAIC_LISTEN_HOST")
        .unwrap_or_else(|_| String::from("127.0.0.1")),
      mosaic_listen_port: env::var("MOSAIC_LISTEN_PORT").unwrap_or_else(|_| String::from("3939")),
      github_token: env::var("GITHUB_TOKEN")
        .unwrap_or_else(|_| panic!("GITHUB_TOKEN is not set, please refer to the documentation.")),
    })
  }
}
