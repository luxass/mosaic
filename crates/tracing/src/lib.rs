use mosaic_utils::AppError;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
  fmt,
  layer::{Layered, SubscriberExt},
  util::SubscriberInitExt,
  EnvFilter, Registry,
};

pub fn init() -> Result<(), AppError> {
  let fmt_layer = fmt_layer();

  #[cfg(feature = "pretty")]
  let fmt_layer = fmt_layer.pretty();

  let registry = tracing_subscriber::registry()
    .with(filter_layer())
    .with(fmt_layer);
  // .with(tracing_error::ErrorLayer::default());

  #[cfg(feature = "console")]
  let registry = registry.with(console_subscriber::spawn());

  registry.init();

  Ok(())
}

fn filter_layer() -> EnvFilter {
  EnvFilter::builder()
    .with_default_directive(LevelFilter::INFO.into())
    .with_env_var("MOSAIC_LOG")
    .from_env_lossy()
}

#[cfg(feature = "json")]
type FmtLayer = fmt::Layer<
  Layered<EnvFilter, Registry>,
  fmt::format::JsonFields,
  fmt::format::Format<fmt::format::Json>,
>;

#[cfg(not(feature = "json"))]
type FmtLayer = fmt::Layer<Layered<EnvFilter, Registry>>;

fn fmt_layer() -> FmtLayer {
  #[cfg(feature = "json")]
  {
    fmt::layer().json()
  }

  #[cfg(not(feature = "json"))]
  {
    fmt::layer()
  }
}
