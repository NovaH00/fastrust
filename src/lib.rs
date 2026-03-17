//! fastrust - A FastAPI-inspired web framework for building APIs in Rust.
//!
//! fastrust provides a simple, intuitive API for building web services with
//! automatic OpenAPI 3.0 specification generation and Swagger UI support.
//!
//! # Example
//!
//! ```rust,no_run
//! use fastrust::{APIApp, APIRouter, RouteConfig};
//! use axum::extract::Path;
//!
//! async fn hello(Path(name): Path<String>) -> String {
//!     format!("Hello {}\n", name)
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut api = APIRouter::new("/api");
//!     api.get("/hello/{name}", hello, RouteConfig::default());
//!
//!     APIApp::new()
//!         .set_title("My API")
//!         .set_port(8080)
//!         .register_router(api)
//!         .run().await;
//! }
//! ```
//!
//! # Features
//!
//! - FastAPI-inspired API design
//! - Automatic OpenAPI 3.0 specification generation
//! - Built-in Swagger UI at `/docs`
//! - Type-safe request/response schemas via `schemars`
//! - Built on top of axum

mod middleware;
mod app;
mod path;
mod extractor;
mod routing;
mod openapi;
mod error;

pub use app::APIApp;
pub use routing::{APIRouter, RouteConfig, Method};
pub use path::canonicalize_path;
pub use extractor::InspectSignature;
pub use error::{Error, Result};
