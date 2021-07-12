//! Phonological unit of speech sound.
//!
//! Phonemes are the abstract blocks into which listeners divide and
//! categorize speech sounds. They are associated with particular languages
//! and vary from accent to accent. This package assumes that the symbolic
//! representation for a phoneme is the International Phonetic Alphabet (IPA).

use crate::features::Segment;

///A Phoneme is a unit of speech sound.
///
///Most phonemes are monosegments, like 'ɪ' or 't'. However, there are also
///instances where a single phoneme behaves like a sequence of two phonological
///segments (a disegment), as in the cases of diphthongs ('a͡ɪ') or affricates
///('t͡ʃ'). This representation does away with the need for a 'delrel' feature
///on segments.
pub enum Phoneme {
    ///A phoneme with a single phonological segment
    Monosegment(Segment),
    ///A phoneme comprised of an ordered sequence of two segments
    Disegment(Segment, Segment),
}

impl Phoneme {
    ///The symbol associated with a phoneme. ex: 'p', 't͡ʃ'
    pub fn symbol(&self) -> String {
        match self {
            Phoneme::Monosegment(s) => s.symbol.clone(),
            Phoneme::Disegment(s1, s2) => {
                format!("{}͡{}", &s1.symbol, &s2.symbol)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Phoneme::{Disegment, Monosegment};
    use super::*;
    use crate::features::*;

    #[test]
    fn symbol_monosegment() {
        let m = Monosegment(mk_segment("p"));
        assert_eq!(m.symbol(), "p")
    }

    #[test]
    fn symbol_disegment() {
        let m = Disegment(mk_segment("t"), mk_segment("ʃ"));
        assert_eq!(m.symbol(), "t͡ʃ")
    }

    // dummy segment constructor; only interested in the symbol.
    fn mk_segment(sym: &str) -> Segment {
        Segment {
            root_features: RootFeatures {
                consonantal: BinaryFeature::Unmarked,
                sonorant: BinaryFeature::Unmarked,
                syllabic: BinaryFeature::Unmarked,
            },
            autosegmental_features: AutosegmentalFeatures {
                nasal: None,
                lateral: None,
                rhotic: None,
                strident: None,
                continuant: None,
                place: None,
                laryngeal: None,
            },
            symbol: sym.to_string(),
        }
    }
}
