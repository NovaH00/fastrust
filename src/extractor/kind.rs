use axum::extract::{Json, Path, State, Query};
use schemars::{JsonSchema, Schema};

/// Represents the kind of Axum extractor used in a handler.
///
/// This enum is used to extract metadata from handler parameters to
/// automatically generate OpenAPI documentation. Each variant corresponds
/// to a different type of data extraction from HTTP requests.
#[derive(Debug)]
pub enum ExtractorKind {
    /// Path parameter extractor (e.g., `Path<i32>` for `/user/{id}`).
    Path(Schema),
    /// JSON body extractor (e.g., `Json<User>` for POST body).
    Json(Schema),
    /// Query parameter extractor (e.g., `Query<Params>` for `?name=value`).
    Query(Schema),
    /// Application state extractor (e.g., `State<AppState>`).
    /// Contains the type name as a string.
    State(&'static str),
}

/// Trait for extracting metadata from Axum extractors.
///
/// This trait is automatically implemented for common Axum extractors
/// and is used internally by fastrust to generate OpenAPI documentation.
///
/// # Examples
///
/// The trait is implemented for standard extractors like:
/// - `Path<T>` where `T: JsonSchema`
/// - `Json<T>` where `T: JsonSchema`
/// - `Query<T>` where `T: JsonSchema`
/// - `State<T>`
pub trait ExtractorMeta {
    /// Returns the extractor kind with associated schema information.
    fn kind() -> ExtractorKind;
}

impl<T> ExtractorMeta for Path<T>
where
    T: JsonSchema,
{
    fn kind() -> ExtractorKind {
        ExtractorKind::Path(schemars::schema_for!(T))
    }
}

impl<T> ExtractorMeta for Json<T>
where
    T: JsonSchema,
{
    fn kind() -> ExtractorKind {
        ExtractorKind::Json(schemars::schema_for!(T))
    }
}

impl<T> ExtractorMeta for State<T> {
    fn kind() -> ExtractorKind {
        ExtractorKind::State(std::any::type_name::<T>())
    }
}

impl<T> ExtractorMeta for Query<T>
where
    T: JsonSchema,
{
    fn kind() -> ExtractorKind {
        ExtractorKind::Query(schemars::schema_for!(T))
    }
}
