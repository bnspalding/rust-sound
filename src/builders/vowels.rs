//! Builders for constructing vowels
//!
//! This module contains a collection of helpful builders for constructing vowels

use crate::features::*;

/// tongue is behind neutral position (forward-back)
///
/// Both back and central vowels are \[+back\].
pub fn back(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default())
        .back = Some(BinaryFeature::Marked)
}

/// tongue body is forward of neutral position (forward-back)
pub fn front(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default())
        .back = Some(BinaryFeature::Unmarked)
}

/// tongue body is near neutral position (forward-back)
///
/// For forward-back contrast, central sounds are \[+back\]
pub fn central(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default())
        .back = Some(BinaryFeature::Marked)
}

/// tongue body is above neutral position (high-low)
pub fn high(s: &mut Segment) {
    let d = s
        .autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default());
    d.low = Some(BinaryFeature::Unmarked);
    d.high = Some(BinaryFeature::Marked);
}

/// tongue body is neither high nor low
pub fn mid(s: &mut Segment) {
    let d = s
        .autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default());
    d.low = Some(BinaryFeature::Unmarked);
    d.high = Some(BinaryFeature::Unmarked);
}

/// tongue body is below neutral position (high-low)
pub fn low(s: &mut Segment) {
    let d = s
        .autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default());
    d.low = Some(BinaryFeature::Marked);
    d.high = Some(BinaryFeature::Unmarked);
}

/// rounding or pursing of the lips
pub fn rounded(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .labial
        .get_or_insert(LabialFeature::default())
        .round = Some(UnaryFeature::Marked)
}

/// lip rounding is absent
///
/// This function is not strictly necessary because roundness is a
/// Unary Feature. Unrounded vowels should be able to simply omit
/// this function in their construction.
pub fn unrounded(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .labial
        .get_or_insert(LabialFeature::default())
        .round = None
}

/// tenseness of sound produced by tongue root raising (ATR) or other means
pub fn tense(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .pharyngeal
        .get_or_insert(PharyngealFeature::default())
        .advanced_tongue_root = Some(BinaryFeature::Marked)
}

/// r-colored sounds produced by a raised or curled tongue tip
pub fn rhotic(s: &mut Segment) {
    s.autosegmental_features.rhotic = Some(UnaryFeature::Marked)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::SegmentBuilder;

    #[test]
    fn test_front() {
        let seg = SegmentBuilder::vowel(&[front], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .dorsal
                .unwrap()
                .back,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_central() {
        let seg = SegmentBuilder::vowel(&[central], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .dorsal
                .unwrap()
                .back,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_back() {
        let seg = SegmentBuilder::vowel(&[back], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .dorsal
                .unwrap()
                .back,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_high() {
        let seg = SegmentBuilder::vowel(&[high], "");
        let d = seg.autosegmental_features.place.unwrap().dorsal.unwrap();
        assert_eq!(d.low, Some(BinaryFeature::Unmarked));
        assert_eq!(d.high, Some(BinaryFeature::Marked));
    }

    #[test]
    fn test_mid() {
        let seg = SegmentBuilder::vowel(&[mid], "");
        let d = seg.autosegmental_features.place.unwrap().dorsal.unwrap();
        assert_eq!(d.low, Some(BinaryFeature::Unmarked));
        assert_eq!(d.high, Some(BinaryFeature::Unmarked));
    }

    #[test]
    fn test_low() {
        let seg = SegmentBuilder::vowel(&[low], "");
        let d = seg.autosegmental_features.place.unwrap().dorsal.unwrap();
        assert_eq!(d.low, Some(BinaryFeature::Marked));
        assert_eq!(d.high, Some(BinaryFeature::Unmarked));
    }

    #[test]
    fn test_rounded() {
        let seg = SegmentBuilder::vowel(&[rounded], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .labial
                .unwrap()
                .round,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_unrounded() {
        let seg = SegmentBuilder::vowel(&[unrounded], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .labial
                .unwrap()
                .round,
            None
        );
    }

    #[test]
    fn test_tense() {
        let seg = SegmentBuilder::vowel(&[tense], "");
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .pharyngeal
                .unwrap()
                .advanced_tongue_root,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_rhotic() {
        let seg = SegmentBuilder::vowel(&[rhotic], "");
        assert_eq!(
            seg.autosegmental_features.rhotic,
            Some(UnaryFeature::Marked)
        );
    }
}
