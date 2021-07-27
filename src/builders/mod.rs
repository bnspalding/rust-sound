//! Builders for constructing segments

pub mod consonants;
pub mod vowels;
pub mod words;

use crate::features::*;

/// Construct a segment using one of the methods implemented on SegmentBuilder.
pub struct SegmentBuilder {}

impl SegmentBuilder {
    /// Construct a new segment from a collection of builder functions.
    ///
    /// Note that builders are applied in the order they are given (left to
    /// right). For cases where two builders modify the same field on a
    /// segment, be sure that the ordering of builders matches your intent.
    ///
    /// In most cases, [`SegmentBuilder::consonant`] or [`SegmentBuilder::vowel`]
    /// are more appropriate for segment construction. Segment is useful for
    /// arbitrary segments and constructs a root_features set that is by
    /// default entirely Unmarked.
    ///
    /// # Examples
    ///
    /// ```
    /// use sound::builders::SegmentBuilder;
    /// use sound::builders::consonants::*;
    /// use sound::features::*;
    ///
    /// let arbitrary_segment =
    ///     SegmentBuilder::segment(&[
    ///         vl,
    ///         stop,
    ///         |s| s.root_features.consonantal = BinaryFeature::Marked,
    ///         |s| s.autosegmental_features
    ///                 .laryngeal
    ///                 .get_or_insert(LaryngealFeatures::default())
    ///                 .spread_glottis = Some(UnaryFeature::Marked),
    ///         ],
    ///         'a');
    /// ```
    pub fn segment(builders: &[fn(&mut Segment)], sym: char) -> Segment {
        let mut base = mk_base(sym);

        for f in builders {
            f(&mut base)
        }

        base
    }

    /// Construct a new consonant (+consonantal, -syllabic) from a collection
    /// of builder functions.
    ///
    /// Consonant generates a (-sonorant) base segment by default. This is
    /// modified by builder functions where appropriate.
    ///
    /// Note that builders are applied in the order they are given (left to
    /// right). For cases where two builders modify the same field on a
    /// segment, be sure that the ordering of builders matches your intent.
    ///
    /// # Examples
    ///
    /// ```
    /// use sound::builders::SegmentBuilder;
    /// use sound::builders::consonants::*;
    /// use sound::phoneme::Phoneme::Monosegment;
    ///
    /// let p = Monosegment(SegmentBuilder::consonant(&[vl, bilabial, stop], 'p'));
    /// ```
    pub fn consonant(builders: &[fn(&mut Segment)], sym: char) -> Segment {
        let mut base = mk_base(sym);
        base.root_features.consonantal = BinaryFeature::Marked;

        for f in builders {
            f(&mut base)
        }

        base
    }

    /// Construct a new vowel (-consonantal, +sonorant, +syllabic) from a collection
    /// of builder functions.
    ///
    /// Note that builders are applied in the order they are given (left to
    /// right). For cases where two builders modify the same field on a
    /// segment, be sure that the ordering of builders matches your intent.
    ///
    /// # Examples
    ///
    /// ```
    /// use sound::builders::SegmentBuilder;
    /// use sound::builders::vowels::*;
    /// use sound::phoneme::Phoneme::Monosegment;
    ///
    /// let i = Monosegment(SegmentBuilder::vowel(&[high, front, tense], 'i'));
    /// ```
    pub fn vowel(builders: &[fn(&mut Segment)], sym: char) -> Segment {
        let mut base = mk_base(sym);
        base.root_features.sonorant = BinaryFeature::Marked;
        base.root_features.syllabic = BinaryFeature::Marked;

        for f in builders {
            f(&mut base)
        }

        base
    }
}

fn mk_base(sym: char) -> Segment {
    Segment {
        root_features: RootFeatures {
            consonantal: BinaryFeature::Unmarked,
            sonorant: BinaryFeature::Unmarked,
            syllabic: BinaryFeature::Unmarked,
        },
        autosegmental_features: AutosegmentalFeatures::default(),
        symbol: sym,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Builders can be arbitrary and apply in order to a segment
    fn test_builder_fns() {
        let seg = SegmentBuilder::segment(
            &[|s| s.symbol = 'x', |s| s.symbol = 'y'],
            'a',
        );
        assert_eq!(seg.symbol, 'y');
    }

    #[test]
    // Segment is by default unmarked and empty
    fn test_segment() {
        let seg = SegmentBuilder::segment(&[], 'p');
        assert_eq!(
            seg,
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
                symbol: 'p',
            }
        );
    }

    #[test]
    // Consonant is by default (+consonantal, -sonorant, -syllabic) and empty
    fn test_consonant() {
        let seg = SegmentBuilder::consonant(&[], 'p');
        assert_eq!(
            seg,
            Segment {
                root_features: RootFeatures {
                    consonantal: BinaryFeature::Marked,
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
                symbol: 'p',
            }
        );
    }

    #[test]
    // Consonant is by default (-consonantal, +sonorant, +syllabic) and empty
    fn test_vowel() {
        let seg = SegmentBuilder::vowel(&[], 'p');
        assert_eq!(
            seg,
            Segment {
                root_features: RootFeatures {
                    consonantal: BinaryFeature::Unmarked,
                    sonorant: BinaryFeature::Marked,
                    syllabic: BinaryFeature::Marked,
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
                symbol: 'p',
            }
        );
    }
}
