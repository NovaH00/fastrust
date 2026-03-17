use axum::routing::MethodRouter;
use std::fmt;
use super::super::extractor::ExtractorKind;
use super::RouteConfig;
use super::openapi_converter::OpenApiConverter;
use openapiv3 as oa;
use openapiv3::Operation;

/// HTTP method enum for route definitions.
///
/// This enum represents the standard HTTP methods supported by fastrust.
/// It is used internally to register routes with the correct HTTP method
/// and to generate OpenAPI specifications.
#[derive(Clone, Debug)]
pub enum Method {
    /// GET - Retrieve a resource
    Get,
    /// POST - Create a resource
    Post,
    /// PUT - Replace a resource
    Put,
    /// PATCH - Partially update a resource
    Patch,
    /// DELETE - Delete a resource
    Delete,
    /// HEAD - Get resource headers
    Head,
    /// OPTIONS - Get supported methods
    Options,
    /// TRACE - Echo the request
    Trace,
    /// CONNECT - Establish a tunnel (not typically used in REST APIs)
    Connect,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Method::Get     => "GET",
            Method::Post    => "POST",
            Method::Put     => "PUT",
            Method::Patch   => "PATCH",
            Method::Delete  => "DELETE",
            Method::Options => "OPTIONS",
            Method::Head    => "HEAD",
            Method::Trace   => "TRACE",
            Method::Connect => "CONNECT"
        };
        write!(f, "{s}")
    }
}

/// Represents a single route in the application.
///
/// A route consists of an HTTP method, a path pattern, a handler, and
/// OpenAPI operation metadata. Routes are created by [`APIRouter`]
/// methods like `get()`, `post()`, etc.
///
/// # Type Parameters
///
/// * `S` - The application state type
#[derive(Clone, Debug)]
pub struct Route<S = ()> {
    /// The HTTP method for this route.
    pub method: Method,
    /// The path pattern (e.g., "/users/{id}").
    pub path: String,
    /// The Axum method router containing the handler.
    pub handler: MethodRouter<S>,
    /// The OpenAPI operation metadata.
    pub operation: oa::Operation,
}

impl<S> Route<S> {
    /// Creates a new route with the given method, path, and handler.
    ///
    /// # Arguments
    ///
    /// * `method` - The HTTP method
    /// * `path` - The path pattern
    /// * `handler` - The Axum method router
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::routing::{Route, Method};
    /// use axum::routing::get;
    ///
    /// async fn handler() -> &'static str { "Hello" }
    ///
    /// let route = Route::new(Method::Get, "/hello".to_string(), get(handler));
    /// ```
    pub fn new(method: Method, path: String, handler: MethodRouter<S>) -> Self {
        Self {
            method,
            path,
            handler,
            operation: Operation::default(),
        }
    }

    /// Configures the OpenAPI operation based on extractor kinds.
    ///
    /// This method analyzes the handler's extractor types and generates
    /// appropriate OpenAPI parameter definitions for path parameters,
    /// query parameters, and request body.
    ///
    /// # Arguments
    ///
    /// * `extractor_kinds` - Vector of extractor metadata from the handler
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_openapi_operation(&mut self, extractor_kinds: Vec<ExtractorKind>) -> &mut Self {
        let mut converter = OpenApiConverter::new();
        for kind in extractor_kinds {
            converter.process_extractor_kind(kind);
        }
        self.operation = converter.into_operation();
        self
    }

    /// Configures the route with metadata and response definitions.
    ///
    /// This method sets the summary, description, tags, and response
    /// schemas for the OpenAPI operation.
    ///
    /// # Arguments
    ///
    /// * `route_config` - Configuration containing metadata and responses
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn set_route_config(&mut self, route_config: RouteConfig) -> &mut Self {
        self.operation.summary = route_config.summary;
        self.operation.description = route_config.description;
        self.operation.tags = route_config.tags;

        let mut responses = indexmap::IndexMap::new();

        for (status, config) in route_config.responses {
            let mut content = indexmap::IndexMap::new();

            if let Some(js_schema) = config.schema {
                let oa_schema = OpenApiConverter::convert_schemars_to_oa(&js_schema);

                content.insert(
                    "application/json".to_string(),
                    oa::MediaType {
                        schema: Some(oa::ReferenceOr::Item(oa_schema)),
                        example: None,
                        examples: indexmap::IndexMap::new(),
                        encoding: indexmap::IndexMap::new(),
                        extensions: indexmap::IndexMap::new(),
                    },
                );
            }

            let response = oa::Response {
                description: config.description,
                content,
                headers: indexmap::IndexMap::new(),
                links: indexmap::IndexMap::new(),
                extensions: indexmap::IndexMap::new(),
            };

            responses.insert(oa::StatusCode::Code(status), oa::ReferenceOr::Item(response));
        }

        self.operation.responses = OpenApiConverter::build_responses(responses);
        self
    }
}
