use std::collections::BTreeMap;
use schemars::{JsonSchema, SchemaGenerator};
use schemars::Schema as JsonSchemaObject;

/// Configuration for a single response in OpenAPI documentation.
///
/// This struct defines the schema and description for a response
/// at a specific HTTP status code.
#[derive(Clone, Debug)]
pub struct ResponseConfig {
    /// Description of the response.
    pub description: String,
    /// Optional JSON schema for the response body.
    pub schema: Option<JsonSchemaObject>,
}

/// Configuration for a route's OpenAPI documentation.
///
/// `RouteConfig` provides a builder pattern for adding metadata,
/// response definitions, and other OpenAPI documentation to routes.
///
/// # Examples
///
/// ```rust
/// use fastrust::RouteConfig;
///
/// #[derive(schemars::JsonSchema, serde::Serialize)]
/// struct User {
///     id: i32,
///     name: String,
/// }
///
/// let config = RouteConfig::default()
///     .summary("Get a user")
///     .description("Returns a user by ID")
///     .tag("users")
///     .response::<User>(200, "User found")
///     .empty_response(404, "User not found");
/// ```
#[derive(Clone, Debug, Default)]
pub struct RouteConfig {
    /// Short summary of the route.
    pub summary: Option<String>,
    /// Long description (supports Markdown).
    pub description: Option<String>,
    /// Tags for grouping routes in documentation.
    pub tags: Vec<String>,
    /// Response definitions by status code.
    pub responses: BTreeMap<u16, ResponseConfig>,
}

impl RouteConfig {
    /// Creates a new empty `RouteConfig`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::RouteConfig;
    ///
    /// let config = RouteConfig::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the route description.
    ///
    /// The description supports Markdown formatting and is displayed
    /// in the Swagger UI.
    ///
    /// # Arguments
    ///
    /// * `description` - The description text
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the route summary.
    ///
    /// The summary is a short title displayed in the Swagger UI.
    ///
    /// # Arguments
    ///
    /// * `summary` - The summary text
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Adds a tag to the route.
    ///
    /// Tags are used to group routes in the Swagger UI. Can be called
    /// multiple times to add multiple tags.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to add
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Adds a 200 OK response with a typed schema.
    ///
    /// This is a convenience method for adding a successful response
    /// with a JSON body schema.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The response type (must implement `JsonSchema`)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::RouteConfig;
    ///
    /// #[derive(schemars::JsonSchema, serde::Serialize)]
    /// struct User { id: i32 }
    ///
    /// let config = RouteConfig::new().ok::<User>();
    /// ```
    pub fn ok<T: JsonSchema>(self) -> Self {
        self.response::<T>(200, "Success")
    }

    /// Adds a response with a specific status code and typed JSON body.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The response type (must implement `JsonSchema`)
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code
    /// * `description` - The response description
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::RouteConfig;
    ///
    /// #[derive(schemars::JsonSchema, serde::Serialize)]
    /// struct User { id: i32 }
    ///
    /// let config = RouteConfig::new().response::<User>(201, "User created");
    /// ```
    pub fn response<T: JsonSchema>(mut self, status: u16, description: impl Into<String>) -> Self {
        let mut generator = SchemaGenerator::default();
        let schema = T::json_schema(&mut generator);

        self.responses.insert(status, ResponseConfig {
            description: description.into(),
            schema: Some(schema),
        });
        self
    }

    /// Adds a response with no body (e.g., 204 No Content, 404 Not Found).
    ///
    /// # Arguments
    ///
    /// * `status` - The HTTP status code
    /// * `description` - The response description
    ///
    /// # Examples
    ///
    /// ```rust
    /// use fastrust::RouteConfig;
    ///
    /// let config = RouteConfig::new().empty_response(204, "No content");
    /// ```
    pub fn empty_response(mut self, status: u16, description: impl Into<String>) -> Self {
        self.responses.insert(status, ResponseConfig {
            description: description.into(),
            schema: None,
        });
        self
    }
}
