//! Structured collection of phonemes.
//!
//! Syllables are broken into three groups of phonemes: onset, nucleus, and
//! coda. The nucleus and the coda together are commonly known as the rhyme.
//! A syllable is also marked with a level of stress, although this really only
//! takes on meaning in relation to other syllables in the same word (lexical
//! stress).

use crate::phoneme::Phoneme;
use crate::stress::Stress;

/// A Syllable describes a structured collection of phonemes, what people commonly
/// distinguish as the unit out of which words are constructed.
#[derive(PartialEq, Eq, Debug)]
pub struct Syllable {
    /// The onset is the collection of phonemes that begin a syllable
    pub onset: Vec<Phoneme>,
    /// The nucleus, normally a vowel, is the most sonorous phoneme in the syl.
    pub nucleus: Phoneme,
    /// The coda is the collection of phonemes that follow the nucleus.
    pub coda: Vec<Phoneme>,
    /// A syllable's stress marks the level of stress that is put on the syllable.
    /// It is only meaningful in relation to surrounding syllables in the same word.
    pub stress: Option<Stress>,
}

impl Syllable {
    /// The rhyme is the nucleus and coda of a syllable together.
    pub fn rhyme(&self) -> Vec<Phoneme> {
        let mut vec = vec![self.nucleus];
        vec.extend(&self.coda);
        vec
    }

    /// phonemes flattens a syllable into a single ordered list of phonemes.
    pub fn phonemes(&self) -> Vec<Phoneme> {
        let mut vec = Vec::new();
        vec.extend(&self.onset);
        vec.push(self.nucleus);
        vec.extend(&self.coda);
        vec
    }

    /// symbols returns the symbolic representation of a syllable's phonemes as a
    /// single String.
    ///
    /// Because stress is only relevant between syllables, it is not rendered
    /// as part of a Syllable's symbols, but instead as part of a Word's symbols.
    pub fn symbols(&self) -> String {
        self.phonemes()
            .iter()
            .fold(String::new(), |acc, p| acc + &p.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accents::genam::phoneme;
    use crate::phoneme::Phoneme;
    use crate::stress::Stress;

    fn test_syl() -> Syllable {
        Syllable {
            onset: vec![phon("p"), phon("ɹ")],
            nucleus: phon("ɑ"),
            coda: vec![phon("p")],
            stress: Some(Stress::Stressed),
        }
    }

    fn phon(s: &str) -> Phoneme {
        phoneme(s).unwrap()
    }

    #[test]
    fn test_rhyme() {
        let test_syl = test_syl();
        assert_eq!(test_syl.rhyme(), vec![phon("ɑ"), phon("p")])
    }

    #[test]
    fn test_phonemes() {
        let test_syl = test_syl();
        assert_eq!(
            test_syl.phonemes(),
            vec![phon("p"), phon("ɹ"), phon("ɑ"), phon("p")]
        )
    }

    #[test]
    fn test_symbols() {
        let test_syl = test_syl();
        assert_eq!(test_syl.symbols(), String::from("pɹɑp"))
    }
}
