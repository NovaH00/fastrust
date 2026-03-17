use schemars::Schema;
use serde_json::Value;

/// Represents the high-level category of a parameter
#[derive(Debug)]
pub enum ParamType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
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


#[derive(Debug)]
pub struct FieldInfo {
    pub name: String,
    pub kind: ParamType,
    pub format: Option<String>, // e.g., "int32", "uuid"
    pub is_required: bool,
}

pub fn schema_to_fields(schema: &Schema) -> Vec<FieldInfo> {
    let val: Value = serde_json::to_value(schema).unwrap_or(Value::Null);
    let mut fields = Vec::new();

    // Determine if we are dealing with a Struct (Object) or a Tuple (Array)
    let schema_type = val.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");

    match schema_type {
        "object" => {
            // 1. Get the list of required field names
            let required_names: Vec<&str> = val.get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                .unwrap_or_default();

            // 2. Map properties to FieldInfo
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
            // Handle Tuples (prefixItems in JSON Schema 2020-12)
            if let Some(items) = val.get("prefixItems").and_then(|v| v.as_array()) {
                for (index, item) in items.iter().enumerate() {
                    fields.push(FieldInfo {
                        name: format!("item_{}", index),
                        kind: ParamType::from(item.get("type").and_then(|v| v.as_str()).unwrap_or("unknown")),
                        format: item.get("format").and_then(|v| v.as_str()).map(|s| s.to_string()),
                        is_required: true, // Elements in a tuple are usually required
                    });
                }
            }
        }
        _ => {
            // Fallback for primitive types if they aren't wrapped in a struct/tuple
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
