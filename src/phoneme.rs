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
#[derive(PartialEq, Eq, Debug, Hash, Clone, Copy)]
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
            Phoneme::Monosegment(s) => s.symbol.to_string(),
            Phoneme::Disegment(s1, s2) => {
                format!("{}͡{}", &s1.symbol, &s2.symbol)
            }
        }
    }
}

impl From<Segment> for Phoneme {
    fn from(seg: Segment) -> Self {
        Phoneme::Monosegment(seg)
    }
}

#[cfg(test)]
mod tests {
    use super::Phoneme::{Disegment, Monosegment};
    use crate::builders::SegmentBuilder;

    #[test]
    fn test_symbol_monosegment() {
        let m = Monosegment(SegmentBuilder::segment(&[], 'p'));
        assert_eq!(m.symbol(), "p")
    }

    #[test]
    fn test_symbol_disegment() {
        let m = Disegment(
            SegmentBuilder::segment(&[], 't'),
            SegmentBuilder::segment(&[], 'ʃ'),
        );
        assert_eq!(m.symbol(), "t͡ʃ")
    }
}
