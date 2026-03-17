use axum::extract::{Json, Path, State, Query};
use schemars::{JsonSchema, Schema};

#[derive(Debug)]
pub enum ExtractorKind {
    Path(Schema),
    Json(Schema),    
    Query(Schema),
    State(&'static str)
}

// We require all extractors to implement this trait so we can identify them
pub trait ExtractorMeta {
    fn kind() -> ExtractorKind;
}

impl<T> ExtractorMeta for Path<T> 
where 
    T: JsonSchema
{
    fn kind() -> ExtractorKind {
        // ExtractorKind::Path(std::any::type_name::<T>())
        ExtractorKind::Path(schemars::schema_for!(T))
    }
}

// Require T to implement JsonSchema!
impl<T> ExtractorMeta for Json<T> 
where 
    T: JsonSchema 
{
    fn kind() -> ExtractorKind {
        // schema_for! macro generates the full structural representation of T
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
    T: JsonSchema
{
    fn kind() -> ExtractorKind {
        ExtractorKind::Query(schemars::schema_for!(T))
    }
}

