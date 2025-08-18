use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{fmt, io};
use tracing::error;

// --- Custom Error Type ---
#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Toml(toml::de::Error),
    MissingKey(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::Toml(e) => write!(f, "TOML parsing error: {}", e),
            Error::MissingKey(key) => write!(f, "Configuration key '{}' not found", key),
        }
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        error!("An internal error occurred: {}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}