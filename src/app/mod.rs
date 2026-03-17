//! API application builder and server.
//!
//! This module provides the [`APIApp`] struct, which is the main entry point
//! for building and running a fastrust application.

mod builder;
mod openapi;
mod swagger;
mod server;

pub use builder::APIApp;
