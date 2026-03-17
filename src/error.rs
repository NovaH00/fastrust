//! Error types for the fastrust framework.

use std::fmt;

/// Error types for fastrust operations.
///
/// This enum represents the various error conditions that can occur
/// when using the fastrust framework.
#[derive(Debug, Clone)]
pub enum Error {
    /// Invalid route configuration (e.g., overlapping routes).
    RouteError(String),
    /// Invalid server configuration (e.g., invalid host/port).
    ServerError(String),
    /// Failed to parse socket address.
    AddressError(String),
    /// Failed to bind to the socket address.
    BindError(String),
    /// OpenAPI specification generation failed.
    OpenApiError(String),
    /// Internal error.
    Internal(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RouteError(msg) => write!(f, "Route error: {msg}"),
            Error::ServerError(msg) => write!(f, "Server error: {msg}"),
            Error::AddressError(msg) => write!(f, "Address error: {msg}"),
            Error::BindError(msg) => write!(f, "Bind error: {msg}"),
            Error::OpenApiError(msg) => write!(f, "OpenAPI error: {msg}"),
            Error::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

/// Result type alias for fastrust operations.
///
/// This is a convenience type alias that uses [`Error`] as the error type.
pub type Result<T> = std::result::Result<T, Error>;
