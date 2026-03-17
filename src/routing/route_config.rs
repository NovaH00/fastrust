use std::collections::BTreeMap;
use schemars::{JsonSchema, SchemaGenerator};
use schemars::Schema as JsonSchemaObject;
use serde::de;

#[derive(Clone, Debug)]
pub struct ResponseConfig {
    pub description: String,
    pub schema: Option<JsonSchemaObject>,
}

#[derive(Clone, Debug, Default)]
pub struct RouteConfig {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub responses: BTreeMap<u16, ResponseConfig>,
}

impl RouteConfig {
    pub fn new() -> Self {
        Self::default()
    }
   
    /// Set a long-form description (supports Markdown)
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
 
    /// Set the summary (the short title in Swagger)
    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Add a tag (groups routes in UI)
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }

    /// Conveniently add a 200 OK response with a body
    pub fn ok<T: JsonSchema>(self) -> Self {
        self.response::<T>(200, "Success")
    }

    /// Add a response with a specific status code and a JSON body
    pub fn response<T: JsonSchema>(mut self, status: u16, description: impl Into<String>) -> Self {
        // This is how you generate a schema from a generic type at runtime
        let mut generator = SchemaGenerator::default();
        let schema = T::json_schema(&mut generator);

        self.responses.insert(status, ResponseConfig {
            description: description.into(),
            schema: Some(schema),
        });
        self
    }

    /// Add a response that has NO body (e.g. 204 No Content or 404 Not Found)
    pub fn empty_response(mut self, status: u16, description: impl Into<String>) -> Self {
        self.responses.insert(status, ResponseConfig {
            description: description.into(),
            schema: None,
        });
        self
    }
}
