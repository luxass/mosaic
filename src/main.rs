use std::{env, path::Path};

use human_panic::{metadata, setup_panic};
use mosaic_utils::{AppEnv, AppError, AppState, GitHubClient};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), AppError> {
  setup_panic!(metadata!().homepage("https://github.com/luxass/mosaic/issues"));

  mosaic_tracing::init()?;

  tracing::info!("loading environment variables");
  if dotenvy::dotenv().is_err() {
    let env_file = env::var("MOSAIC_ENV_FILE").unwrap_or_else(|_| String::from("/etc/mosaic/.dev"));
    if dotenvy::from_path(Path::new(&env_file)).is_err() {
      tracing::debug!("no .env file found");
    }
  }

  let env = AppEnv::init()?;

  let db = PgPoolOptions::new()
    .max_connections(20)
    .connect(&env.database_url)
    .await?;

  sqlx::migrate!().run(&db).await.unwrap();

  let state = AppState {
    env: env.clone(),
    github: GitHubClient::new(&env.github_token)?,
    db,
  };

  tracing::info!("starting server");
  mosaic_server::run(state).await
}
