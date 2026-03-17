//! Extractor metadata and signature inspection.
//!
//! This module provides traits and types for inspecting Axum extractors
//! to automatically generate OpenAPI parameter documentation.

mod kind;
mod inspect;

pub use kind::ExtractorKind;
pub use inspect::InspectSignature;
