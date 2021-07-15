//! GenAm Sound Definitions
//!
//! This module contains the actual GenAm mappings from symbols to phonemes.

use crate::builders::consonants::*;
use crate::builders::vowels::*;
use crate::builders::*;
use crate::features::Segment;
use crate::phoneme::Phoneme;
use crate::phoneme::Phoneme::*;
use std::collections::HashMap;

use lazy_static::lazy_static;
use maplit::hashmap;

lazy_static! {
    pub static ref SOUNDS: HashMap<&'static str, Phoneme> = hashmap! {
        //Consonants
        "m" => Monosegment(consonant(&[vd, bilabial, nasal], "m")),
        "n" => Monosegment(consonant(&[vd, alveolar, nasal], "n")),
        "ŋ" => Monosegment(consonant(&[vd, velar, nasal], "ŋ")),

        // Vowels
        "i" => Monosegment(vowel(&[high, front, tense], "i")),
        "ɪ" => Monosegment(vowel(&[high, front], "ɪ")),
    };
}

fn consonant(fs: &[fn(&mut Segment)], sym: &str) -> Segment {
    SegmentBuilder::consonant(fs, sym)
}

fn vowel(fs: &[fn(&mut Segment)], sym: &str) -> Segment {
    SegmentBuilder::vowel(fs, sym)
}
