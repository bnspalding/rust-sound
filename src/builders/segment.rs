//! Base builder methods for Segments

use crate::features::*;

impl Segment {
    /// Construct a new segment from a collection of builder functions.
    ///
    /// Note that builders are applied in the order they are given (left to
    /// right). For cases where two builders modify the same field on a
    /// segment, be sure that the ordering of builders matches your intent.
    pub fn new(builders: &[fn(&mut Segment)], sym: &str) -> Segment {
        let mut base = mk_segment();

        base.symbol = sym.to_string();

        for f in builders {
            f(&mut base)
        }

        base
    }
}

fn mk_segment() -> Segment {
    Segment {
        root_features: RootFeatures {
            consonantal: BinaryFeature::Unmarked,
            sonorant: BinaryFeature::Unmarked,
            syllabic: BinaryFeature::Unmarked,
        },
        autosegmental_features: AutosegmentalFeatures {
            nasal: None,
            lateral: None,
            rhotic: None,
            strident: None,
            continuant: None,
            place: None,
            laryngeal: None,
        },
        symbol: String::new(),
    }
}
