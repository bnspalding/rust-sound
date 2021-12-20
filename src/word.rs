//! A collection of Syllables
//!
//! A word is an ordered list of syllables. It also provides functions for
//! flattening the sounds and stresses of a word into lists.

use crate::phoneme::Phoneme;
use crate::stress::Stress;
use crate::syllable::Syllable;

/// A word is a collection of syllables. It represents a spoken word, or
/// perhaps the pronunciation information that would correspond to a written
/// word.
#[derive(PartialEq, Eq, Debug)]
pub struct Word(Vec<Syllable>);

impl Word {
    /// phonemes combines the flattened phoneme sets of a word's syllables into
    /// a single vector. The structure (onset-nucleus-coda) of the syllables is
    /// lost in this transformation.
    pub fn phonemes(self) -> Vec<Phoneme> {
        let mut vec = Vec::new();

        for syl in self {
            vec.extend(syl.phonemes())
        }

        vec
    }

    /// stresses provides the list of stress levels corresponding to each syl
    /// in a word.
    pub fn stresses(self) -> Vec<Stress> {
        let mut vec = Vec::new();

        for syl in self {
            if let Some(stress) = syl.stress {
                vec.push(stress)
            }
        }

        vec
    }

    /// symbols returns a textual representation of a syllabized word
    ///
    /// Syllables are separated by the '.' character, except for syllables that
    /// begin with an IPA stress mark, which serves as a syllable separator in
    /// place of the dot.
    pub fn symbols(self) -> String {
        if self.0.is_empty() {
            return String::from("");
        }

        let mut syms = String::new();

        for (i, syl) in self.into_iter().enumerate() {
            let separator = syl
                .stress
                .unwrap_or(Stress::Unstressed)
                .symbol()
                .unwrap_or('.');
            if i != 0 || separator != '.' {
                syms.push(separator);
            }
            syms.push_str(&syl.symbols());
        }

        syms
    }
}

impl From<Vec<Syllable>> for Word {
    fn from(syls: Vec<Syllable>) -> Word {
        Word(syls)
    }
}
/*
impl Into<Vec<Syllable>> for Word {
    fn into(self) -> Vec<Syllable> {
        self.0
    }
}
*/

impl IntoIterator for Word {
    type Item = Syllable;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::accents::genam::phoneme;

    fn mk_syl(
        onset: Vec<Phoneme>,
        nucleus: Phoneme,
        coda: Vec<Phoneme>,
        stress: Stress,
    ) -> Syllable {
        Syllable {
            onset,
            nucleus,
            coda,
            stress: Some(stress),
        }
    }

    fn phon(s: &str) -> Phoneme {
        phoneme(s).unwrap()
    }

    fn phons(ss: Vec<&str>) -> Vec<Phoneme> {
        let mut vec = Vec::new();
        for s in ss {
            vec.push(phon(s));
        }
        vec
    }

    fn test_word() -> Word {
        Word::from(vec![
            mk_syl(
                phons(vec!["p"]),
                phon("ʌ"),
                phons(vec!["m", "p"]),
                Stress::Stressed,
            ),
            mk_syl(
                phons(vec!["k"]),
                phon("ɪ"),
                phons(vec!["n"]),
                Stress::Unstressed,
            ),
        ])
    }

    #[test]
    fn test_phonemes() {
        let test_word = test_word();
        assert_eq!(
            test_word.phonemes(),
            phons(vec!["p", "ʌ", "m", "p", "k", "ɪ", "n"])
        )
    }

    #[test]
    fn test_stresses() {
        let test_word = test_word();
        assert_eq!(
            test_word.stresses(),
            vec![Stress::Stressed, Stress::Unstressed]
        )
    }

    #[test]
    fn test_symbols() {
        let test_word = test_word();
        assert_eq!(test_word.symbols(), String::from("ˈpʌmp.kɪn"))
    }
}
