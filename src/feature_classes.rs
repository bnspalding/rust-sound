//! Natural classes of phonemes from features
//!
//! Feature_Classes provides functions for testing the membership of a phoneme in various natural
//! classes without requiring the user to understand the specific features that define that class
//! or the structure of features within the phoneme. In cases where the user needs to know "is this
//! phoneme an X (vowel, nasal, fricative, ...)" it is better to use these functions than to
//! construct the test ad hoc.

use crate::features::{BinaryFeature, Segment, UnaryFeature};
use crate::phoneme::Phoneme;

/// A vowel is any phoneme that goes in the nucleus of a syllable. They are marked +syllabic.
pub fn is_vowel(p: Phoneme) -> bool {
    any_segment(p, |seg| seg.root_features.syllabic == BinaryFeature::Marked)
}

/// A consonant is any phoneme that does not go in the nucleus of a syllable. They are marked
/// -syllabic. This definition of a consonant is meant to contrast with the notion of a vowel, and
/// therefore includes semivowels even though they are -consonantal.
pub fn is_consonant(p: Phoneme) -> bool {
    !is_vowel(p)
}

/// A semivowel is a phoneme with (-consonantal, -syllabic) features.
pub fn is_semivowel(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.syllabic == BinaryFeature::Unmarked
            && seg.root_features.consonantal == BinaryFeature::Unmarked
    })
}

/// A voiced phoneme is +voiced
pub fn is_voiced(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.autosegmental_features
            .laryngeal
            .and_then(|laryn| laryn.voice)
            .map_or(false, |voice| voice == BinaryFeature::Marked)
    })
}

/// A stop is a phoneme with (-sonorant, -continuant) features
pub fn is_stop(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.sonorant == BinaryFeature::Unmarked
            && seg
                .autosegmental_features
                .continuant
                .map_or(false, |continuant| {
                    continuant == BinaryFeature::Unmarked
                })
    })
}

/// A fricative is a phoneme with (-sonorant, +continuant) features
pub fn is_fricative(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        &&seg.root_features.sonorant == BinaryFeature::Unmarked
            && seg
                .autosegmental_features
                .continuant
                .map_or(false, |continuant| continuant == BinaryFeature::Marked)
    })
}

/// An approximant is a phoneme with (+sonorant, -syllabic, +continuant) features
pub fn is_approximant(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.sonorant == BinaryFeature::Marked
            && seg.root_features.syllabic == BinaryFeature::Unmarked
            && seg
                .autosegmental_features
                .continuant
                .map_or(false, |continuant| continuant == BinaryFeature::Marked)
    })
}

/// An affricate is a disegment in which the first segment is a stop and the second is a fricative.
pub fn is_affricate(p: Phoneme) -> bool {
    match p {
        Phoneme::Monosegment(_) => false,
        Phoneme::Disegment(seg1, seg2) => {
            is_stop(seg1.into()) && is_fricative(seg2.into())
        }
    }
}

/// A nasal is a phoneme with the nasal feature
pub fn is_nasal(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.autosegmental_features
            .nasal
            .map_or(false, |nasal| nasal == UnaryFeature::Marked)
    })
}

/// A lateral is a phoneme with the lateral feature
pub fn is_lateral(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.autosegmental_features
            .lateral
            .map_or(false, |lateral| lateral == UnaryFeature::Marked)
    })
}

/// A high vowel is a phoneme with (+syllabic, +high) features
pub fn is_high_vowel(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.syllabic == BinaryFeature::Marked
            && seg
                .autosegmental_features
                .place
                .and_then(|place| place.dorsal)
                .and_then(|dorsal| dorsal.high)
                .map_or(false, |high| high == BinaryFeature::Marked)
    })
}

/// A low vowel is a phoneme with (+syllabic, +low) features
pub fn is_low_vowel(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.syllabic == BinaryFeature::Marked
            && seg
                .autosegmental_features
                .place
                .and_then(|place| place.dorsal)
                .and_then(|dorsal| dorsal.low)
                .map_or(false, |low| low == BinaryFeature::Marked)
    })
}

/// A mid vowel is a phoneme with (+syllabic, -high, -low) features
pub fn is_mid_vowel(p: Phoneme) -> bool {
    any_segment(p, |seg| {
        seg.root_features.syllabic == BinaryFeature::Marked
            && seg
                .autosegmental_features
                .place
                .and_then(|place| place.dorsal)
                .and_then(|dorsal| dorsal.high)
                .map_or(false, |high| high == BinaryFeature::Unmarked)
            && seg
                .autosegmental_features
                .place
                .and_then(|place| place.dorsal)
                .and_then(|dorsal| dorsal.low)
                .map_or(false, |low| low == BinaryFeature::Unmarked)
    })
}

fn any_segment(p: Phoneme, f: fn(Segment) -> bool) -> bool {
    match p {
        Phoneme::Monosegment(seg) => f(seg),
        Phoneme::Disegment(seg1, seg2) => f(seg1) || f(seg2),
    }
}
