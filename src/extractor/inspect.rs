use super::kind::{ExtractorKind, ExtractorMeta};

/// Trait for inspecting handler function signatures.
///
/// This trait is automatically implemented for handler function signatures
/// (tuples of extractors) and is used to extract metadata for OpenAPI
/// documentation generation.
///
/// # Examples
///
/// The trait is implemented for handler signatures like:
/// - `fn handler() -> String` (no extractors)
/// - `fn handler(Path(i32)) -> String` (single extractor)
/// - `fn handler(Path(i32), Json<User>) -> String` (multiple extractors)
pub trait InspectSignature {
    /// Returns a vector of extractor kinds for this handler signature.
    fn extractors() -> Vec<ExtractorKind>;
}

impl InspectSignature for ((),) {
    fn extractors() -> Vec<ExtractorKind> {
        vec![]
    }
}

macro_rules! impl_inspect_signature {
    () => {};
    ( $head:ident $(, $tail:ident)* ) => {
        impl<M, $head $(, $tail)*> InspectSignature for (M, $head $(, $tail)*)
        where
            $head: ExtractorMeta,
            $( $tail: ExtractorMeta ),*
        {
            fn extractors() -> Vec<ExtractorKind> {
                vec![
                    <$head as ExtractorMeta>::kind(),
                    $( <$tail as ExtractorMeta>::kind() ),*
                ]
            }
        }
        impl_inspect_signature!( $($tail),* );
    };
}

impl_inspect_signature!(
    T16, T15, T14, T13, T12, T11, T10, T9, T8, T7, T6, T5, T4, T3, T2, T1
);
