//! Similarity between phoneme collections

use crate::feature_set::{feature_set, Feature};
use crate::phoneme::Phoneme;
use crate::syllable::Syllable;
use std::collections::HashSet;

/// Rhyme generates a measure of similarity between two syllables by comparing the rhymes of the
/// syllables (nucleus + coda).
pub fn rhyme(syl1: Syllable, syl2: Syllable) -> f64 {
    similarity(&syl1.rhyme()[..], &syl2.rhyme()[..])
}

/// Assonance generates a measure of similarity between two syllables by comparing their nueclei.
pub fn assonance(syl1: Syllable, syl2: Syllable) -> f64 {
    similarity(&[syl1.nucleus], &[syl2.nucleus])
}

/// Alliteration generates a measure of similarity between two syllables by comparing their onsets.
pub fn alliteration(syl1: Syllable, syl2: Syllable) -> f64 {
    similarity(&syl1.onset[..], &syl2.onset[..])
}

/// Similarity is a measure between 0 (completely different) and 1 (exactly the same) for two
/// collections of phonemes. This measure is the fraction of features shared by the two sets over
/// the total number of unique features in the two phoneme sets.
pub fn similarity(s1: &[Phoneme], s2: &[Phoneme]) -> f64 {
    let fs1 = gather_features(s1);
    let fs2 = gather_features(s2);
    let shared_features =
        fs1.intersection(&fs2).collect::<HashSet<&Feature>>().len() as f64;
    let all_features =
        fs1.union(&fs2).collect::<HashSet<&Feature>>().len() as f64;

    shared_features / all_features
}

fn gather_features(phonemes: &[Phoneme]) -> HashSet<Feature> {
    let mut features = HashSet::new();
    for phoneme in phonemes {
        features = &features | &feature_set(*phoneme)
    }

    features
}

//TODO: write tests for approx functions
