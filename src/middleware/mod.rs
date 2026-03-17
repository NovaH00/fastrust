//! Middleware components for fastrust.
//!
//! This module provides built-in middleware for cross-cutting concerns
//! like logging, authentication, and request/response transformation.

mod log_request;

pub use log_request::log_request;
