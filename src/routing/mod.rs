//! Routing and route configuration.
//!
//! This module provides the [`APIRouter`] struct for defining routes and
//! the [`RouteConfig`] struct for configuring route metadata and OpenAPI
//! documentation.

mod route;
mod router;
mod route_config;
mod openapi_converter;

pub use router::APIRouter;
pub use route::Method;
pub use route_config::RouteConfig;
