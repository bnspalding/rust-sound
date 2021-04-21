//! Phonemes
//!
//! TODO: write module documentation

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
