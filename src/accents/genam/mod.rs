//! GenAm Sound Definitions
//!
//! This module provides a mapping from a set of IPA symbols to a set of
//! phonemes, based on the '[General American English]' accent.
//!
//! [General American English]: <https://en.wikipedia.org/wiki/General_American_English>

use crate::builders::words::{from_accent, WordConstructorError};
use crate::phoneme::Phoneme;
use crate::word::Word;
use std::collections::HashSet;

mod sounds;

/// phoneme provides a constructor for General American English phonemes.
/// Given the IPA symbol for a phoneme, return the associated phoneme
/// (if one exists).
///
/// # Examples
///
/// ```
/// # use sound::accents::genam::phoneme;
/// # use sound::builders::SegmentBuilder;
/// # use sound::builders::consonants::*;
/// # use sound::phoneme::Phoneme::Monosegment;
///
/// let p = phoneme("p");
///
/// assert_eq!(
///     p,
///     Some(Monosegment(SegmentBuilder::consonant(&[vl, bilabial, stop], 'p')))
/// )
/// ```
pub fn phoneme(symbol: &str) -> Option<Phoneme> {
    sounds::SOUNDS.get(symbol).cloned()
}

/// The set of IPA symbols that comprise the GenAm accent
pub fn symbols() -> HashSet<&'static str> {
    sounds::SOUNDS.keys().copied().collect()
}

/// The set of Phonemes that comprise the GenAm accent
pub fn phonemes() -> HashSet<&'static Phoneme> {
    sounds::SOUNDS.values().collect()
}

/// word provides a constructor for syllable-structured groups of General American English
/// phonemes. Given a collection of IPA symbols for the sounds of the word, return either a Word
/// comprised of those phonemes or a [WordConstructorError].
///
/// See [from_accent] for more information, as this function is simply
/// a GenAm wrapper for that function.
///
/// # Examples
///
/// ```
/// # use sound::accents::genam::{phoneme, word};
/// # use sound::syllable::Syllable;
/// # use sound::word::Word;
/// # use sound::stress::Stress;
///
/// let w = word("ˈhɛ.lo͡ʊ").unwrap();
///
/// assert_eq!(
///     w,
///     Word::new(&[
///         Syllable::new(&[phoneme("h").unwrap()], phoneme("ɛ").unwrap(), &[], Some(Stress::Stressed)),
///         Syllable::new(&[phoneme("l").unwrap()], phoneme("o͡ʊ").unwrap(), &[], Some(Stress::Unstressed)),
///         ])
/// );
///
/// ```
pub fn word(word_desc: &str) -> Result<Word, WordConstructorError> {
    from_accent(phoneme, word_desc)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::features::*;
    use std::iter::FromIterator;

    #[test]
    fn test_symbols() {
        assert_eq!(
            symbols(),
            HashSet::from_iter(vec![
                "m", "n", "ŋ", "p", "b", "t", "d", "k", "ɡ", "t͡ʃ", "d͡ʒ", "f",
                "v", "θ", "ð", "s", "z", "ʃ", "ʒ", "h", "l", "ɹ", "j", "ʍ",
                "w", "i", "ɪ", "ɛ", "ə", "æ", "ʌ", "ɑ", "u", "ʊ", "ɔ", "e͡ɪ",
                "a͡ɪ", "a͡ʊ", "o͡ʊ", "ɔ͡ɪ", "ɜ˞", "ə˞"
            ])
        );
    }

    #[test]
    fn test_phoneme_m() {
        let m = phoneme("m");
        assert!(m.is_some());
        if let Phoneme::Monosegment(seg) = m.unwrap() {
            assert_eq!(
                seg.autosegmental_features.nasal,
                Some(UnaryFeature::Marked)
            );
            assert_eq!(
                seg.autosegmental_features.place.as_ref().unwrap().labial,
                Some(LabialFeature::default())
            );
            assert_eq!(seg.root_features.sonorant, BinaryFeature::Marked);
        }
    }

    #[test]
    fn test_phoneme_ei() {
        let m = phoneme("e͡ɪ");
        assert!(m.is_some());
        if let Phoneme::Disegment(seg1, seg2) = m.unwrap() {
            assert_eq!(
                seg1.autosegmental_features
                    .place
                    .as_ref()
                    .unwrap()
                    .pharyngeal,
                Some(PharyngealFeature {
                    advanced_tongue_root: Some(BinaryFeature::Marked)
                }),
            );
            assert_eq!(
                seg2.autosegmental_features
                    .place
                    .as_ref()
                    .unwrap()
                    .pharyngeal,
                Some(PharyngealFeature {
                    advanced_tongue_root: Some(BinaryFeature::Unmarked)
                }),
            );
            assert_eq!(
                seg1.autosegmental_features
                    .place
                    .as_ref()
                    .unwrap()
                    .dorsal
                    .as_ref()
                    .unwrap()
                    .back,
                Some(BinaryFeature::Unmarked)
            );
            assert_eq!(
                seg2.autosegmental_features
                    .place
                    .as_ref()
                    .unwrap()
                    .dorsal
                    .as_ref()
                    .unwrap()
                    .back,
                Some(BinaryFeature::Unmarked)
            );
        }
    }
}
