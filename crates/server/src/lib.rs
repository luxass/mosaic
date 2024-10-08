use axum::Router;
use mosaic_utils::{AppError, AppState};
use tokio::net::TcpListener;
use tower_http::{
  cors::CorsLayer,
  trace::{self, TraceLayer},
};
use tracing::Level;

mod openapi;
mod routes;

pub async fn run(state: AppState) -> Result<(), AppError> {
  let env = state.env.clone();

  tracing::info!("creating app");
  let app = Router::new()
    .nest("/", routes::routes())
    .with_state(state.clone())
    .layer(CorsLayer::permissive())
    .layer(
      TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
    );

  let http = async {
    let listener = TcpListener::bind(format!(
      "{}:{}",
      env.mosaic_listen_host, env.mosaic_listen_port
    ))
    .await?;
    tracing::debug!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
      .with_graceful_shutdown(async {
        mosaic_utils::shutdown_signal()
          .await
          .expect("failed to install graceful shutdown handler")
      })
      .await?;

    Ok::<(), AppError>(())
  };

  let cron = mosaic_cron::run(state);

  let _res = tokio::join!(http, cron);

  Ok(())
}
