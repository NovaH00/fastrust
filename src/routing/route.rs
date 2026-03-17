use axum::{handler, routing::MethodRouter};
use std::{fmt, process};
use super::super::inspector::ExtractorKind;
use super::super::docs_generator::{
    FieldInfo,
    schema_to_fields,
    ParamType,
    param_type_to_openapi_v3 
};
use super::RouteConfig;

use indexmap::IndexMap;
use schemars::{Schema};
use openapiv3 as oa;
use openapiv3::{
    IntegerType, MediaType, OpenAPI, Operation, PathItem, ReferenceOr, RequestBody, Response,
    SchemaKind, StatusCode, StringType, Type, ObjectType, Info,
    Parameter, ParameterData, ParameterSchemaOrContent // Corrected types here
};

#[derive(Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
    Trace,
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

#[derive(Clone, Debug)]
pub struct Route<S = ()> {
    pub method: Method,
    pub path: String,
    pub handler: MethodRouter<S>,
    pub operation: Operation, // OpenAPI specs
}

impl<S> Route<S> {
    pub fn new(method: Method, path: String, handler: MethodRouter<S>) -> Self {
        Self {
            method,
            path,
            handler,
            operation: Operation::default(),
        }
    }

    fn process_extractor_kind(&mut self, extractor_kind: ExtractorKind) {
        match extractor_kind {
            ExtractorKind::Path(schema) => {
                for field in schema_to_fields(&schema) {
                    let path_parameter = Parameter::Path { 
                        parameter_data: ParameterData { 
                            name: field.name,
                            description: None,
                            deprecated: None,
                            required: field.is_required,
                            format: param_type_to_openapi_v3(&field.kind, &field.format),
                            example: None,
                            examples: IndexMap::new(),
                            explode: None,
                            extensions: IndexMap::new()

                        },
                        style: Default::default(),
                    }; 
                    self.operation.parameters.push(ReferenceOr::Item(path_parameter));
                }
            },
            ExtractorKind::Json(json_schema) => {
                let mut properties = IndexMap::new();
                let mut required = Vec::new();

                // 1. Process the fields from the schemars schema
                for field in schema_to_fields(&json_schema) {

                    let field_schema_or_content = param_type_to_openapi_v3(&field.kind, &field.format);

                    // Convert ReferenceOr<Schema> to ReferenceOr<Box<Schema>>
                    if let oa::ParameterSchemaOrContent::Schema(s) = field_schema_or_content {
                        let boxed_schema = match s {
                            oa::ReferenceOr::Reference { reference } => {
                                oa::ReferenceOr::Reference { reference }
                            }
                            oa::ReferenceOr::Item(item) => {
                                oa::ReferenceOr::Item(Box::new(item))
                            }
                        };

                        properties.insert(field.name.clone(), boxed_schema);

                        if field.is_required {
                            required.push(field.name);
                        }
                    }
                }

                // 2. Build the Body Schema using explicit openapiv3 types
                let body_schema = oa::Schema {
                    schema_data: oa::SchemaData::default(),
                    schema_kind: oa::SchemaKind::Type(oa::Type::Object(oa::ObjectType {
                        properties,
                        required,
                        additional_properties: None,
                        min_properties: None,
                        max_properties: None,
                    })),
                };

                // 3. Create the Media Type
                let media_type = oa::MediaType {
                    schema: Some(oa::ReferenceOr::Item(body_schema)),
                    example: None,
                    examples: IndexMap::new(),
                    encoding: IndexMap::new(),
                    extensions: IndexMap::new(),
                };

                // 4. Create the Request Body
                let mut content = IndexMap::new();
                content.insert("application/json".to_string(), media_type);

                let request_body = oa::RequestBody {
                    content,
                    description: Some("JSON payload".to_string()),
                    required: true, 
                    extensions: IndexMap::new(),
                };

                self.operation.request_body = Some(oa::ReferenceOr::Item(request_body));
            },
            ExtractorKind::Query(schema) => {
                for field in schema_to_fields(&schema) {
                    let query_parameter = oa::Parameter::Query {
                        parameter_data: oa::ParameterData {
                            name: field.name,
                            description: None,
                            required: field.is_required, // Will be false if using Option<T>
                            deprecated: None,
                            format: param_type_to_openapi_v3(&field.kind, &field.format),
                            example: None,
                            examples: IndexMap::new(),
                            explode: None, 
                            extensions: IndexMap::new(),
                        },
                        // 'Form' is the standard style for ?name=val&age=10
                        style: oa::QueryStyle::Form,
                        allow_reserved: false,
                        allow_empty_value: None,
                    };

                    self.operation.parameters.push(oa::ReferenceOr::Item(query_parameter));
                }
            }, 
            ExtractorKind::State(_) => ()  
        }
    }

    pub fn set_openapi_operation(&mut self, extractor_kinds: Vec<ExtractorKind>) -> &mut Self {
        for arg in extractor_kinds {
            self.process_extractor_kind(arg); 
        }
        self
    }

    pub fn set_route_config(&mut self, route_config: RouteConfig) -> &mut Self {
        // 1. Simple Metadata
        self.operation.summary = route_config.summary;
        self.operation.description = route_config.description;
        self.operation.tags = route_config.tags;

        // 2. Clear and rebuild Responses
        let mut oa_responses = oa::Responses {
            default: None,
            responses: indexmap::IndexMap::new(),
            extensions: indexmap::IndexMap::new(),
        };

        for (status, config) in route_config.responses {
            let mut content = indexmap::IndexMap::new();

            // 3. If a schema exists, wrap it in application/json media type
            if let Some(js_schema) = config.schema {
                // Convert schemars::Schema -> openapiv3::Schema
                let oa_schema = self.convert_schemars_to_oa(&js_schema);

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

            oa_responses.responses.insert(
                oa::StatusCode::Code(status),
                oa::ReferenceOr::Item(response),
            );
        }

        self.operation.responses = oa_responses;
        self
    }

    /// Internal helper to turn a Response Struct Schema into an OpenAPI Object Schema
    fn convert_schemars_to_oa(&self, js_schema: &schemars::Schema) -> oa::Schema {
        // Use your existing generator logic to get the fields of the response struct
        let fields = schema_to_fields(js_schema);

        let mut properties = indexmap::IndexMap::new();
        let mut required = Vec::new();

        for field in fields {
            // Use your existing converter logic
            let field_oa = param_type_to_openapi_v3(&field.kind, &field.format);

            if let oa::ParameterSchemaOrContent::Schema(s) = field_oa {
                // Properties in OpenAPI objects must be Boxed
                let boxed = match s {
                    oa::ReferenceOr::Reference { reference } => oa::ReferenceOr::Reference { reference },
                    oa::ReferenceOr::Item(i) => oa::ReferenceOr::Item(Box::new(i)),
                };

                properties.insert(field.name.clone(), boxed);
                if field.is_required {
                    required.push(field.name);
                }
            }
        }

        oa::Schema {
            schema_data: oa::SchemaData::default(),
            schema_kind: oa::SchemaKind::Type(oa::Type::Object(oa::ObjectType {
                properties,
                required,
                additional_properties: None,
                min_properties: None,
                max_properties: None,
            })),
        }
    }
}


