//! Builders for constructing consonants
//!
//! This module contains a collection of helpful builders for constructing consonants

use crate::features::*;

/// a voiced segment
pub fn vd(s: &mut Segment) {
    s.autosegmental_features
        .laryngeal
        .get_or_insert(LaryngealFeatures::default())
        .voice = Some(BinaryFeature::Marked)
}

/// a voiceless segment
pub fn vl(s: &mut Segment) {
    s.autosegmental_features
        .laryngeal
        .get_or_insert(LaryngealFeatures::default())
        .voice = Some(BinaryFeature::Unmarked)
}

/// a consonant that is (-continuant, -sonorant)
pub fn stop(s: &mut Segment) {
    s.root_features.sonorant = BinaryFeature::Unmarked;
    s.autosegmental_features.continuant = Some(BinaryFeature::Unmarked);
}

/// a consonant that is (+sonorant, -continuant, nasal)
pub fn nasal(s: &mut Segment) {
    s.root_features.sonorant = BinaryFeature::Marked;
    s.autosegmental_features.continuant = Some(BinaryFeature::Unmarked);
    s.autosegmental_features.nasal = Some(UnaryFeature::Marked);
}

/// a consonant that is (-sonorant, +continuant, -strident)
pub fn fricative(s: &mut Segment) {
    s.root_features.sonorant = BinaryFeature::Unmarked;
    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
    s.autosegmental_features.strident = Some(BinaryFeature::Unmarked);
}

/// a semivowel (-consonantal, +sonorant, -syllabic, +continuant)
pub fn glide(s: &mut Segment) {
    s.root_features.sonorant = BinaryFeature::Marked;
    s.root_features.consonantal = BinaryFeature::Unmarked;
    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
}

/// a consonant that is (+sonorant, +continuant)
pub fn approximant(s: &mut Segment) {
    s.root_features.sonorant = BinaryFeature::Marked;
    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
}

/// a segment marked +strident
pub fn sibilant(s: &mut Segment) {
    s.autosegmental_features.strident = Some(BinaryFeature::Marked);
}

/// a segment marked +distrib
pub fn distrib(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .coronal
        .get_or_insert(CoronalFeature::default())
        .distrib = Some(BinaryFeature::Marked)
}

/// a segment marked lateral
pub fn lateral(s: &mut Segment) {
    s.autosegmental_features.lateral = Some(UnaryFeature::Marked);
}

/// a segment marked rhotic
pub fn rhotic(s: &mut Segment) {
    s.autosegmental_features.rhotic = Some(UnaryFeature::Marked);
}

/// a labially articulated segment
pub fn bilabial(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .labial
        .get_or_insert(LabialFeature::default());
}

/// a labially articulated segment. From the perspective of distinctive
/// features, this is marked the same as [bilabial].
pub fn labiodental(s: &mut Segment) {
    bilabial(s);
}

/// a coronally articulated segment marked (+anterior, -distrib)
pub fn alveolar(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .coronal
        .replace(CoronalFeature {
            anterior: Some(BinaryFeature::Marked),
            distrib: Some(BinaryFeature::Unmarked),
        });
}

/// a coronally articulated segment marked (+anterior). From the perspective of
/// distinctive features, this is marked the same as [alveolar].
pub fn dental(s: &mut Segment) {
    alveolar(s);
}

/// a coronally articulated segment marked (-anterior, -distrib)
pub fn postalveolar(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .coronal
        .replace(CoronalFeature {
            anterior: Some(BinaryFeature::Unmarked),
            distrib: Some(BinaryFeature::Unmarked),
        });
}

/// a dorsally articulated segment
pub fn velar(s: &mut Segment) {
    s.autosegmental_features
        .place
        .get_or_insert(Place::default())
        .dorsal
        .get_or_insert(DorsalFeature::default());
}

/// a dorsally articulated segment. From the perspective of distinctive
/// features, this is marked the same as [velar].
pub fn palatal(s: &mut Segment) {
    velar(s);
}

/// a segment with laryngeal constriction
pub fn glottal(s: &mut Segment) {
    s.autosegmental_features
        .laryngeal
        .get_or_insert(LaryngealFeatures::default());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::SegmentBuilder;

    #[test]
    fn test_vd() {
        let seg = SegmentBuilder::consonant(&[vd], 'a');
        assert_eq!(
            seg.autosegmental_features.laryngeal.unwrap().voice,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_vl() {
        let seg = SegmentBuilder::consonant(&[vl], 'a');
        assert_eq!(
            seg.autosegmental_features.laryngeal.unwrap().voice,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_stop() {
        let seg = SegmentBuilder::consonant(&[stop], 'a');
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Unmarked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_nasal() {
        let seg = SegmentBuilder::consonant(&[nasal], 'a');
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Marked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Unmarked)
        );
        assert_eq!(
            seg.autosegmental_features.nasal,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_fricative() {
        let seg = SegmentBuilder::consonant(&[fricative], 'a');
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Unmarked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Marked)
        );
        assert_eq!(
            seg.autosegmental_features.strident,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_glide() {
        let seg = SegmentBuilder::consonant(&[glide], 'a');
        assert_eq!(seg.root_features.consonantal, BinaryFeature::Unmarked);
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Marked);
        assert_eq!(seg.root_features.syllabic, BinaryFeature::Unmarked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_approximant() {
        let seg = SegmentBuilder::consonant(&[approximant], 'a');
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Marked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_sibilant() {
        let seg = SegmentBuilder::consonant(&[sibilant], 'a');
        assert_eq!(
            seg.autosegmental_features.strident,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_distrib() {
        let seg = SegmentBuilder::consonant(&[distrib], 'a');
        assert_eq!(
            seg.autosegmental_features
                .place
                .unwrap()
                .coronal
                .unwrap()
                .distrib,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_lateral() {
        let seg = SegmentBuilder::consonant(&[lateral], 'a');
        assert_eq!(
            seg.autosegmental_features.lateral,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_rhotic() {
        let seg = SegmentBuilder::consonant(&[rhotic], 'a');
        assert_eq!(
            seg.autosegmental_features.rhotic,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_bilabial() {
        let seg = SegmentBuilder::consonant(&[bilabial], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().labial,
            Some(LabialFeature { round: None })
        );
    }

    #[test]
    fn test_labiodental() {
        let seg = SegmentBuilder::consonant(&[labiodental], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().labial,
            Some(LabialFeature { round: None })
        );
    }

    #[test]
    fn test_alveolar() {
        let seg = SegmentBuilder::consonant(&[alveolar], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().coronal,
            Some(CoronalFeature {
                anterior: Some(BinaryFeature::Marked),
                distrib: Some(BinaryFeature::Unmarked),
            })
        );
    }

    #[test]
    fn test_dental() {
        let seg = SegmentBuilder::consonant(&[dental], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().coronal,
            Some(CoronalFeature {
                anterior: Some(BinaryFeature::Marked),
                distrib: Some(BinaryFeature::Unmarked),
            })
        );
    }

    #[test]
    fn test_postalveolar() {
        let seg = SegmentBuilder::consonant(&[postalveolar], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().coronal,
            Some(CoronalFeature {
                anterior: Some(BinaryFeature::Unmarked),
                distrib: Some(BinaryFeature::Unmarked),
            })
        );
    }

    #[test]
    fn test_velar() {
        let seg = SegmentBuilder::consonant(&[velar], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().dorsal,
            Some(DorsalFeature {
                high: None,
                low: None,
                back: None,
            })
        );
    }

    #[test]
    fn test_palatal() {
        let seg = SegmentBuilder::consonant(&[palatal], 'a');
        assert_eq!(
            seg.autosegmental_features.place.unwrap().dorsal,
            Some(DorsalFeature {
                high: None,
                low: None,
                back: None,
            })
        );
    }

    #[test]
    fn test_glottal() {
        let seg = SegmentBuilder::consonant(&[glottal], 'a');
        assert_eq!(
            seg.autosegmental_features.laryngeal,
            Some(LaryngealFeatures {
                voice: None,
                spread_glottis: None,
                constricted_glottis: None,
            })
        );
    }
}
