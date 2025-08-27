use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, thiserror::Error, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(String),
    #[error("Configuration error: {0}")]
    Config(String),
    #[error("Missing configuration key: {0}")]
    MissingKey(&'static str),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Config(err.to_string())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status = match self {
            Error::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Config(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MissingKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}
