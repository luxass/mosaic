use std::{io, num::NonZeroU16};

use axum::{http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, Deserialize, Serialize)]
pub struct WrappedStatusCode(NonZeroU16);

impl From<StatusCode> for WrappedStatusCode {
  fn from(status: StatusCode) -> Self {
      WrappedStatusCode(NonZeroU16::new(status.as_u16()).expect("status code is always non-zero"))
  }
}

#[derive(Debug, Deserialize)]
pub struct GitHubErrorBody {
  pub documentation_url: Option<String>,
  pub errors: Option<Vec<serde_json::Value>>,
  pub message: String,
}

impl std::fmt::Display for GitHubErrorBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    tracing::error!("GitHub error: {:?}", self);
    write!(f, "an error occurred")
  }
}

#[derive(Error, Debug, ToSchema)]
pub enum AppError {
  #[error("IO error: {0}")]
  IoError(#[from] io::Error),

  #[error("Reqwest error: {0}")]
  ReqwestError(#[from] reqwest::Error),

  #[error("axum http error: {0}")]
  Axum(#[from] axum::http::Error),

  #[error("Serde JSON error: {0}")]
  SerdeJsonError(#[from] serde_json::Error),

  #[error("GitHub error: {0}")]
  GitHubError(GitHubErrorBody),

  #[error("SQLx error: {0}")]
  SqlxError(#[from] sqlx::Error),

  #[error("unknown error")]
  Unknown,

  #[error("Not found")]
  NotFound,

  // TODO: Add more details to TaskError
  #[error("Task error: {0}")]
  TaskError(String),

  #[error("parse config error: {0}")]
  ParseConfigError(String),

  #[error("config error: {0}")]
  ResolveConfigError(String),
}


#[derive(Serialize, Deserialize, ToSchema)]
pub struct ApiError {
  /// The Error Message
  pub message: String,
  #[schema(example = "status = 500", examples("400", "401", "403", "404", "500"))]
  /// The HTTP Status Code
  pub status: WrappedStatusCode,
  #[serde(default = "default_timestamp")]
  /// The timestamp of the error
  pub timestamp: DateTime<Utc>,
}

fn default_timestamp() -> DateTime<Utc> {
  Utc::now()
}

pub type ApiErrorResponse = (StatusCode, Json<ApiError>);

impl From<AppError> for ApiErrorResponse {
  fn from(err: AppError) -> Self {
    let status_code = StatusCode::INTERNAL_SERVER_ERROR;

    let payload: Json<ApiError> = Json::from(ApiError {
      message: err.to_string(),
      status: status_code.into(),
      timestamp: chrono::offset::Utc::now(),
    });
    (status_code, payload)
  }
}
