use super::extractor::{ExtractorKind, ExtractorMeta};

pub trait InspectSignature {
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
            // Enforce that every argument implements our trait!
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
