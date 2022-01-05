//! Representing sets of phonological features

use crate::features::Segment;
use crate::phoneme::Phoneme;
use std::collections::HashSet;

#[allow(missing_docs)]
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
/// Phonological Features for representation in a set
pub enum Feature {
    PlusSyllabic,
    MinusSyllabic,
    PlusConsonantal,
    MinusConsonantal,
    PlusSonorant,
    MinusSonorant,
    PlusContinuant,
    MinusContinuant,
    PlusStrident,
    MinusStrident,
    Nasal,
    Lateral,
    Rhotic,
    Laryngeal,
    PlusVoice,
    MinusVoice,
    SpreadGlottis,
    ConstrictedGlottis,
    Labial,
    Round,
    Coronal,
    PlusAnterior,
    MinusAnterior,
    PlusDistrib,
    MinusDistrib,
    Dorsal,
    PlusHigh,
    MinusHigh,
    PlusLow,
    MinusLow,
    PlusBack,
    MinusBack,
    Pharyngeal,
    PlusAdvancedTongueRoot,
    MinusAdvancedTongueRoot,
    DelRel,
}

/// Generate a feature set representing the phonological features of a phoneme.
pub fn feature_set(phoneme: Phoneme) -> HashSet<Feature> {
    use crate::feature_classes::is_affricate;
    use Feature::DelRel;

    match phoneme {
        Phoneme::Monosegment(seg) => segment_feature_set(seg),
        Phoneme::Disegment(seg1, seg2) => {
            let mut features =
                &segment_feature_set(seg1) | &segment_feature_set(seg2);
            if is_affricate(phoneme) {
                features.insert(DelRel);
            }
            features
        }
    }
}

fn segment_feature_set(seg: Segment) -> HashSet<Feature> {
    use crate::features::accessors::*;
    use crate::features::{BinaryFeature, UnaryFeature};
    use Feature::*;

    // Unfortunately, I don't think there's a better way to do this than a chunk of conditionals.
    // Generally, I want to be moving away from feature sets — that's the whole reason to use the
    // structured feature approach — but they're still a very useful tool for things like
    // similarity measurements.

    let mut features = HashSet::new();

    // Root Features
    match seg.root_features.syllabic {
        BinaryFeature::Marked => features.insert(PlusSyllabic),
        BinaryFeature::Unmarked => features.insert(MinusSyllabic),
    };
    match seg.root_features.sonorant {
        BinaryFeature::Marked => features.insert(PlusSonorant),
        BinaryFeature::Unmarked => features.insert(MinusSonorant),
    };
    match seg.root_features.consonantal {
        BinaryFeature::Marked => features.insert(PlusConsonantal),
        BinaryFeature::Unmarked => features.insert(MinusConsonantal),
    };

    // Place Features
    if let Some(labial) = get_labial(seg) {
        features.insert(Labial);
        if let Some(UnaryFeature::Marked) = labial.round {
            features.insert(Round);
        }
    }
    if let Some(coronal) = get_coronal(seg) {
        features.insert(Coronal);
        if let Some(anterior) = coronal.anterior {
            match anterior {
                BinaryFeature::Marked => features.insert(PlusAnterior),
                BinaryFeature::Unmarked => features.insert(MinusAnterior),
            };
        }
        if let Some(distrib) = coronal.distrib {
            match distrib {
                BinaryFeature::Marked => features.insert(PlusDistrib),
                BinaryFeature::Unmarked => features.insert(MinusDistrib),
            };
        }
    }
    if let Some(dorsal) = get_dorsal(seg) {
        features.insert(Dorsal);
        if let Some(high) = dorsal.high {
            match high {
                BinaryFeature::Marked => features.insert(PlusHigh),
                BinaryFeature::Unmarked => features.insert(MinusHigh),
            };
        }
        if let Some(low) = dorsal.low {
            match low {
                BinaryFeature::Marked => features.insert(PlusLow),
                BinaryFeature::Unmarked => features.insert(MinusLow),
            };
        }
        if let Some(back) = dorsal.back {
            match back {
                BinaryFeature::Marked => features.insert(PlusBack),
                BinaryFeature::Unmarked => features.insert(MinusBack),
            };
        }
    }
    if let Some(pharyngeal) = get_pharyngeal(seg) {
        features.insert(Pharyngeal);
        if let Some(atr) = pharyngeal.advanced_tongue_root {
            match atr {
                BinaryFeature::Marked => {
                    features.insert(PlusAdvancedTongueRoot)
                }
                BinaryFeature::Unmarked => {
                    features.insert(MinusAdvancedTongueRoot)
                }
            };
        }
    }

    // Non-place Autosegmental Features
    if let Some(continuant) = get_continuant(seg) {
        match continuant {
            BinaryFeature::Marked => features.insert(PlusContinuant),
            BinaryFeature::Unmarked => features.insert(MinusContinuant),
        };
    }
    if let Some(strident) = get_strident(seg) {
        match strident {
            BinaryFeature::Marked => features.insert(PlusStrident),
            BinaryFeature::Unmarked => features.insert(MinusStrident),
        };
    }
    if let Some(UnaryFeature::Marked) = get_nasal(seg) {
        features.insert(Nasal);
    }
    if let Some(UnaryFeature::Marked) = get_lateral(seg) {
        features.insert(Lateral);
    }
    if let Some(UnaryFeature::Marked) = get_rhotic(seg) {
        features.insert(Rhotic);
    }
    if let Some(laryngeal) = get_laryngeal(seg) {
        features.insert(Laryngeal);
        if let Some(UnaryFeature::Marked) = laryngeal.spread_glottis {
            features.insert(SpreadGlottis);
        }
        if let Some(UnaryFeature::Marked) = laryngeal.constricted_glottis {
            features.insert(ConstrictedGlottis);
        }
        if let Some(voice) = laryngeal.voice {
            match voice {
                BinaryFeature::Marked => features.insert(PlusVoice),
                BinaryFeature::Unmarked => features.insert(MinusVoice),
            };
        }
    }

    features
}
