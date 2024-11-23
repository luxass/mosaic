use std::io;

use axum::Json;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

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

  #[error("Octocrab: {0}")]
  Octocrab(#[from] Box<octocrab::Error>),

  #[error("SQLx error: {0}")]
  SqlxError(#[from] sqlx::Error),

  #[error("unknown error")]
  Unknown,

  #[error("Not found")]
  NotFound,

  #[error("Ignored project")]
  IgnoredProject,

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

  #[serde(skip)]
  pub status_code: StatusCode,

  #[schema(example = "status = 500", examples("400", "401", "403", "404", "500"))]
  /// The HTTP Status Code
  pub status: u16,

  #[serde(default = "default_timestamp")]
  /// The timestamp of the error
  pub timestamp: DateTime<Utc>,
}

fn default_timestamp() -> DateTime<Utc> {
  Utc::now()
}

pub type ApiErrorResponse = (StatusCode, Json<ApiError>);

impl From<&AppError> for StatusCode {
  fn from(value: &AppError) -> Self {
    match value {
      AppError::NotFound => StatusCode::NOT_FOUND,
      AppError::IgnoredProject => StatusCode::FORBIDDEN,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

impl From<AppError> for ApiErrorResponse {
  fn from(err: AppError) -> Self {
    let status_code = StatusCode::from(&err);
    let payload: Json<ApiError> = Json::from(ApiError {
      message: err.to_string(),
      status_code,
      status: status_code.as_u16(),
      timestamp: Utc::now(),
    });
    (status_code, payload)
  }
}

impl From<octocrab::Error> for AppError {
  fn from(error: octocrab::Error) -> Self {
    AppError::Octocrab(Box::new(error))
  }
}
