//! GenAm Sound Definitions
//!
//! This module provides a mapping from a set of IPA symbols to a set of
//! phonemes, based on the 'General American English' accent (see
//! [https://en.wikipedia.org/wiki/General_American_English]).

use crate::phoneme::Phoneme;
use std::collections::HashSet;

mod sounds;

/// phoneme provides a constructor for General American English phonemes.
/// Given the IPA symbol for a phoneme, return the associated phoneme
/// (if one exists).
pub fn phoneme(symbol: &str) -> Option<&'static Phoneme> {
    sounds::SOUNDS.get(symbol)
}

/// The set of IPA symbols that comprise the GenAm accent
pub fn symbols() -> HashSet<&'static str> {
    sounds::SOUNDS.keys().copied().collect()
}

/// The set of Phonemes that comprise the GenAm accent
pub fn phonemes() -> HashSet<&'static Phoneme> {
    sounds::SOUNDS.values().collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_symbols() {
        assert_eq!(
            symbols(),
            HashSet::from_iter(vec![
                "m", "n", "ŋ", "p", "b", "t", "d", "k", "g", "t͡ʃ", "d͡ʒ", "f",
                "v", "θ", "ð", "s", "z", "ʃ", "ʒ", "h", "l", "ɹ", "j", "ʍ",
                "w", "i", "ɪ", "ɛ", "ə", "ʌ", "ɑ", "u", "ʊ", "ɔ", "e͡ɪ", "a͡ɪ",
                "a͡ʊ", "o͡ʊ", "ɔ͡ɪ", "ɜ˞", "ə˞"
            ])
        );
    }
}
