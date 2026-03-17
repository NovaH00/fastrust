use super::super::extractor::ExtractorKind;
use super::super::openapi::{schema_to_fields, param_type_to_openapi_schema};
use indexmap::IndexMap;
use openapiv3 as oa;
use openapiv3::{
    Operation, Parameter, ParameterData, ReferenceOr, Response,
    QueryStyle, StatusCode,
};

/// Converter for building OpenAPI operations from extractor metadata.
///
/// This struct handles the conversion of Axum extractor information
/// into OpenAPI parameter and request body definitions.
pub struct OpenApiConverter {
    /// The OpenAPI operation being built.
    operation: Operation,
}

impl OpenApiConverter {
    /// Creates a new empty converter.
    ///
    /// # Returns
    ///
    /// A new `OpenApiConverter` with an empty operation.
    pub fn new() -> Self {
        Self {
            operation: Operation::default(),
        }
    }

    /// Processes an extractor kind and adds appropriate OpenAPI definitions.
    ///
    /// This method analyzes the extractor type and adds corresponding
    /// OpenAPI parameter or request body definitions to the operation.
    ///
    /// # Arguments
    ///
    /// * `extractor_kind` - The extractor metadata to process
    ///
    /// # Extractor Handling
    ///
    /// - `Path` → Adds path parameters
    /// - `Query` → Adds query parameters
    /// - `Json` → Adds request body schema
    /// - `State` → Ignored (not part of OpenAPI spec)
    pub fn process_extractor_kind(&mut self, extractor_kind: ExtractorKind) {
        match extractor_kind {
            ExtractorKind::Path(schema) => {
                for field in schema_to_fields(&schema) {
                    let path_parameter = Parameter::Path {
                        parameter_data: ParameterData {
                            name: field.name,
                            description: None,
                            deprecated: None,
                            required: field.is_required,
                            format: param_type_to_openapi_schema(&field.kind, &field.format),
                            example: None,
                            examples: IndexMap::new(),
                            explode: None,
                            extensions: IndexMap::new(),
                        },
                        style: Default::default(),
                    };
                    self.operation.parameters.push(ReferenceOr::Item(path_parameter));
                }
            },
            ExtractorKind::Json(json_schema) => {
                let mut properties = IndexMap::new();
                let mut required = Vec::new();

                for field in schema_to_fields(&json_schema) {
                    let field_schema_or_content = param_type_to_openapi_schema(&field.kind, &field.format);

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

                let media_type = oa::MediaType {
                    schema: Some(oa::ReferenceOr::Item(body_schema)),
                    example: None,
                    examples: IndexMap::new(),
                    encoding: IndexMap::new(),
                    extensions: IndexMap::new(),
                };

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
                            required: field.is_required,
                            deprecated: None,
                            format: param_type_to_openapi_schema(&field.kind, &field.format),
                            example: None,
                            examples: IndexMap::new(),
                            explode: None,
                            extensions: IndexMap::new(),
                        },
                        style: QueryStyle::Form,
                        allow_reserved: false,
                        allow_empty_value: None,
                    };

                    self.operation.parameters.push(oa::ReferenceOr::Item(query_parameter));
                }
            },
            ExtractorKind::State(_) => ()
        }
    }

    /// Builds an OpenAPI responses object from a map of status codes to responses.
    ///
    /// # Arguments
    ///
    /// * `responses` - Map of status codes to response definitions
    ///
    /// # Returns
    ///
    /// An OpenAPI `Responses` object ready to be attached to an operation.
    pub fn build_responses(responses: IndexMap<StatusCode, ReferenceOr<Response>>) -> oa::Responses {
        oa::Responses {
            default: None,
            responses,
            extensions: IndexMap::new(),
        }
    }

    /// Converts a schemars schema into an OpenAPI object schema.
    ///
    /// This method is used to convert response type schemas into
    /// OpenAPI format for documentation.
    ///
    /// # Arguments
    ///
    /// * `js_schema` - The schemars schema to convert
    ///
    /// # Returns
    ///
    /// An OpenAPI schema object representing the response structure.
    pub fn convert_schemars_to_oa(js_schema: &schemars::Schema) -> oa::Schema {
        let fields = schema_to_fields(js_schema);

        let mut properties = indexmap::IndexMap::new();
        let mut required = Vec::new();

        for field in fields {
            let field_oa = param_type_to_openapi_schema(&field.kind, &field.format);

            if let oa::ParameterSchemaOrContent::Schema(s) = field_oa {
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

    /// Consumes the converter and returns the built operation.
    ///
    /// # Returns
    ///
    /// The completed OpenAPI operation with all parameters and
    /// request body definitions.
    pub fn into_operation(self) -> Operation {
        self.operation
    }
}
