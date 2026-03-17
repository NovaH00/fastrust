//! OpenAPI specification generation utilities.
//!
//! This module provides types and functions for converting Rust types
//! into OpenAPI 3.0 specification components.

mod generator;
mod converter;

pub use generator::{ParamType, schema_to_fields};
pub use converter::param_type_to_openapi_schema;
