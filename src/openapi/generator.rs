use schemars::Schema;
use serde_json::Value;

/// Represents the high-level type category of a parameter in OpenAPI.
///
/// This enum maps to the primitive types supported by OpenAPI 3.0.
#[derive(Debug)]
pub enum ParamType {
    /// String type (e.g., `String`, `&str`).
    String,
    /// Integer type (e.g., `i32`, `i64`, `u32`).
    Integer,
    /// Number type (e.g., `f32`, `f64`).
    Number,
    /// Boolean type (`bool`).
    Boolean,
    /// Array type (e.g., `Vec<T>`, slices).
    Array,
    /// Object type (e.g., structs, `HashMap<K, V>`).
    Object,
    /// Unknown or unsupported type.
    Unknown,
}

impl From<&str> for ParamType {
    fn from(s: &str) -> Self {
        match s {
            "string" => ParamType::String,
            "integer" => ParamType::Integer,
            "number" => ParamType::Number,
            "boolean" => ParamType::Boolean,
            "array" => ParamType::Array,
            "object" => ParamType::Object,
            _ => ParamType::Unknown,
        }
    }
}

/// Information about a field extracted from a schema.
///
/// This struct contains metadata about a single field that can be used
/// to generate OpenAPI parameter or schema definitions.
#[derive(Debug)]
pub struct FieldInfo {
    /// The name of the field.
    pub name: String,
    /// The type category of the field.
    pub kind: ParamType,
    /// Optional format specifier (e.g., "int32", "uuid", "date-time").
    pub format: Option<String>,
    /// Whether the field is required.
    pub is_required: bool,
}

/// Extracts field information from a JSON schema.
///
/// This function analyzes a [`schemars::Schema`] and extracts structured
/// field information that can be used to generate OpenAPI parameter
/// definitions or schema objects.
///
/// # Arguments
///
/// * `schema` - The JSON schema to analyze
///
/// # Returns
///
/// A vector of [`FieldInfo`] structs, one for each field found in the schema.
///
/// # Examples
///
/// For a struct schema, returns field information for each property.
/// For a tuple/array schema, returns information for each element.
pub fn schema_to_fields(schema: &Schema) -> Vec<FieldInfo> {
    let val: Value = serde_json::to_value(schema).unwrap_or(Value::Null);
    let mut fields = Vec::new();

    let schema_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");

    match schema_type {
        "object" => {
            let required_names: Vec<&str> = val.get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            if let Some(properties) = val.get("properties").and_then(|p| p.as_object()) {
                for (name, details) in properties {
                    fields.push(FieldInfo {
                        name: name.clone(),
                        kind: ParamType::from(details.get("type").and_then(|v| v.as_str()).unwrap_or("unknown")),
                        format: details.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        is_required: required_names.contains(&name.as_str()),
                    });
                }
            }
        }
        "array" => {
            if let Some(items) = val.get("prefixItems").and_then(|v| v.as_array()) {
                for (index, item) in items.iter().enumerate() {
                    fields.push(FieldInfo {
                        name: format!("item_{}", index),
                        kind: ParamType::from(item.get("type").and_then(|v| v.as_str()).unwrap_or("unknown")),
                        format: item.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        is_required: true,
                    });
                }
            }
        }
        _ => {
            fields.push(FieldInfo {
                name: "value".to_string(),
                kind: ParamType::from(schema_type),
                format: val.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                is_required: true,
            });
        }
    }

    fields
}
