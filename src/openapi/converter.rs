use super::ParamType;
use indexmap::IndexMap;
use openapiv3::{
    ArrayType, BooleanType, IntegerFormat, IntegerType, NumberFormat, NumberType,
    ParameterSchemaOrContent, ReferenceOr, Schema, SchemaData, SchemaKind, StringFormat,
    StringType, Type, VariantOrUnknownOrEmpty, ObjectType,
};

/// Converts a [`ParamType`] into an OpenAPI v3 `ParameterSchemaOrContent`.
///
/// This function is used internally by fastrust to convert extracted field
/// type information into OpenAPI schema definitions.
///
/// # Arguments
///
/// * `kind` - The parameter type category
/// * `format` - Optional format specifier (e.g., "int32", "uuid")
///
/// # Returns
///
/// An OpenAPI `ParameterSchemaOrContent` that can be used in parameter
/// or schema definitions.
pub fn param_type_to_openapi_schema(kind: &ParamType, format: &Option<String>) -> ParameterSchemaOrContent {
    let schema_kind = match kind {
        ParamType::Integer => SchemaKind::Type(Type::Integer(IntegerType {
            format: format_to_integer(format),
            multiple_of: None,
            maximum: None,
            exclusive_maximum: false,
            minimum: None,
            exclusive_minimum: false,
            enumeration: vec![],
        })),
        ParamType::String => SchemaKind::Type(Type::String(StringType {
            format: format_to_string(format),
            pattern: None,
            enumeration: vec![],
            min_length: None,
            max_length: None,
        })),
        ParamType::Number => SchemaKind::Type(Type::Number(NumberType {
            format: format_to_number(format),
            multiple_of: None,
            maximum: None,
            exclusive_maximum: false,
            minimum: None,
            exclusive_minimum: false,
            enumeration: vec![],
        })),
        ParamType::Boolean => SchemaKind::Type(Type::Boolean(BooleanType {
            enumeration: vec![],
        })),
        ParamType::Array => SchemaKind::Type(Type::Array(ArrayType {
            items: Some(ReferenceOr::Item(Box::new(Schema {
                schema_data: SchemaData::default(),
                schema_kind: SchemaKind::Any(Default::default()),
            }))),
            min_items: None,
            max_items: None,
            unique_items: false,
        })),
        ParamType::Object | ParamType::Unknown => SchemaKind::Type(Type::Object(ObjectType {
            properties: IndexMap::new(),
            required: vec![],
            additional_properties: None,
            min_properties: None,
            max_properties: None,
        })),
    };

    ParameterSchemaOrContent::Schema(ReferenceOr::Item(Schema {
        schema_data: SchemaData::default(),
        schema_kind,
    }))
}

/// Converts an optional format string to an OpenAPI integer format.
fn format_to_integer(f: &Option<String>) -> VariantOrUnknownOrEmpty<IntegerFormat> {
    match f {
        Some(s) if s == "int32" => VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32),
        Some(s) if s == "int64" => VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64),
        Some(s) => VariantOrUnknownOrEmpty::Unknown(s.clone()),
        None => VariantOrUnknownOrEmpty::Empty,
    }
}

/// Converts an optional format string to an OpenAPI string format.
fn format_to_string(f: &Option<String>) -> VariantOrUnknownOrEmpty<StringFormat> {
    match f {
        Some(s) if s == "date" => VariantOrUnknownOrEmpty::Item(StringFormat::Date),
        Some(s) if s == "date-time" => VariantOrUnknownOrEmpty::Item(StringFormat::DateTime),
        Some(s) if s == "password" => VariantOrUnknownOrEmpty::Item(StringFormat::Password),
        Some(s) if s == "byte" => VariantOrUnknownOrEmpty::Item(StringFormat::Byte),
        Some(s) if s == "binary" => VariantOrUnknownOrEmpty::Item(StringFormat::Binary),
        Some(s) => VariantOrUnknownOrEmpty::Unknown(s.clone()),
        None => VariantOrUnknownOrEmpty::Empty,
    }
}

/// Converts an optional format string to an OpenAPI number format.
fn format_to_number(f: &Option<String>) -> VariantOrUnknownOrEmpty<NumberFormat> {
    match f {
        Some(s) if s == "float" => VariantOrUnknownOrEmpty::Item(NumberFormat::Float),
        Some(s) if s == "double" => VariantOrUnknownOrEmpty::Item(NumberFormat::Double),
        Some(s) => VariantOrUnknownOrEmpty::Unknown(s.clone()),
        None => VariantOrUnknownOrEmpty::Empty,
    }
}
