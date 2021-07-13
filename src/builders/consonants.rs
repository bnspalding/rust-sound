//! Builders for constructing consonants
//!
//! This module contains a collection of helpful builders for constructing consonants

// m = vd bilabial nasal
// m = SegmentBuilder::new(vec![vd, bilabial, nasal])
// example: let p = SegmentBuilder::new().vd().

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
    s.root_features = RootFeatures {
        consonantal: BinaryFeature::Marked,
        sonorant: BinaryFeature::Unmarked,
        syllabic: BinaryFeature::Unmarked,
    };

    s.autosegmental_features.continuant = Some(BinaryFeature::Unmarked);
}

/// a consonant that is (+sonorant, -continuant, nasal)
pub fn nasal(s: &mut Segment) {
    s.root_features = RootFeatures {
        consonantal: BinaryFeature::Marked,
        sonorant: BinaryFeature::Marked,
        syllabic: BinaryFeature::Unmarked,
    };

    s.autosegmental_features.continuant = Some(BinaryFeature::Unmarked);
    s.autosegmental_features.nasal = Some(UnaryFeature::Marked);
}

/// a consonant that is (-sonorant, +continuant, -strident)
pub fn fricative(s: &mut Segment) {
    s.root_features = RootFeatures {
        consonantal: BinaryFeature::Marked,
        sonorant: BinaryFeature::Unmarked,
        syllabic: BinaryFeature::Unmarked,
    };

    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
    s.autosegmental_features.strident = Some(BinaryFeature::Unmarked);
}

/// a semivowel (-consonantal, +sonorant, -syllabic, +continuant)
pub fn glide(s: &mut Segment) {
    s.root_features = RootFeatures {
        consonantal: BinaryFeature::Unmarked,
        sonorant: BinaryFeature::Marked,
        syllabic: BinaryFeature::Unmarked,
    };

    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
}

/// a consonant that is (+sonorant, +continuant)
pub fn approximant(s: &mut Segment) {
    s.root_features = RootFeatures {
        consonantal: BinaryFeature::Marked,
        sonorant: BinaryFeature::Marked,
        syllabic: BinaryFeature::Unmarked,
    };

    s.autosegmental_features.continuant = Some(BinaryFeature::Marked);
}

/// a segment marked +strident
pub fn strident(s: &mut Segment) {
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

    #[test]
    fn test_vd() {
        let seg = Segment::new(&[vd], "");
        assert_eq!(
            seg.autosegmental_features.laryngeal.unwrap().voice,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_vl() {
        let seg = Segment::new(&[vl], "");
        assert_eq!(
            seg.autosegmental_features.laryngeal.unwrap().voice,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_stop() {
        let seg = Segment::new(&[stop], "");
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Unmarked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Unmarked)
        );
    }

    #[test]
    fn test_nasal() {
        let seg = Segment::new(&[nasal], "");
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
        let seg = Segment::new(&[fricative], "");
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
        let seg = Segment::new(&[glide], "");
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
        let seg = Segment::new(&[approximant], "");
        assert_eq!(seg.root_features.sonorant, BinaryFeature::Marked);
        assert_eq!(
            seg.autosegmental_features.continuant,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_strident() {
        let seg = Segment::new(&[strident], "");
        assert_eq!(
            seg.autosegmental_features.strident,
            Some(BinaryFeature::Marked)
        );
    }

    #[test]
    fn test_distrib() {
        let seg = Segment::new(&[distrib], "");
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
        let seg = Segment::new(&[lateral], "");
        assert_eq!(
            seg.autosegmental_features.lateral,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_rhotic() {
        let seg = Segment::new(&[rhotic], "");
        assert_eq!(
            seg.autosegmental_features.rhotic,
            Some(UnaryFeature::Marked)
        );
    }

    #[test]
    fn test_bilabial() {
        let seg = Segment::new(&[bilabial], "");
        assert_eq!(
            seg.autosegmental_features.place.unwrap().labial,
            Some(LabialFeature { round: None })
        );
    }

    #[test]
    fn test_labiodental() {
        let seg = Segment::new(&[labiodental], "");
        assert_eq!(
            seg.autosegmental_features.place.unwrap().labial,
            Some(LabialFeature { round: None })
        );
    }

    #[test]
    fn test_alveolar() {
        let seg = Segment::new(&[alveolar], "");
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
        let seg = Segment::new(&[dental], "");
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
        let seg = Segment::new(&[postalveolar], "");
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
        let seg = Segment::new(&[velar], "");
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
        let seg = Segment::new(&[palatal], "");
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
        let seg = Segment::new(&[glottal], "");
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
