//! GenAm Sound Definitions
//!
//! This module contains the actual GenAm mappings from symbols to phonemes.

use crate::builders::consonants::rhotic as rhotic_c;
use crate::builders::consonants::*;
use crate::builders::vowels::rhotic as rhotic_v;
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
        "m"  => Monosegment(consonant(&[vd, bilabial, nasal], 'm')),
        "n"  => Monosegment(consonant(&[vd, alveolar, nasal], 'n')),
        "ŋ"  => Monosegment(consonant(&[vd, velar, nasal], 'ŋ')),
        "p"  => Monosegment(consonant(&[vl, bilabial, stop], 'p')),
        "b"  => Monosegment(consonant(&[vd, bilabial, stop], 'b')),
        "t"  => Monosegment(consonant(&[vl, alveolar, stop], 't')),
        "d"  => Monosegment(consonant(&[vd, alveolar, stop], 'd')),
        "k"  => Monosegment(consonant(&[vl, velar, stop], 'k')),
        "ɡ"  => Monosegment(consonant(&[vd, velar, stop], 'ɡ')),
        "t͡ʃ" =>   Disegment(consonant(&[vl, alveolar, stop], 't'),
                            consonant(&[vl, postalveolar, distrib, sibilant, fricative], 'ʃ')),
        "d͡ʒ" =>   Disegment(consonant(&[vd, alveolar, stop], 'd'),
                            consonant(&[vd, postalveolar, distrib, sibilant, fricative], 'ʒ')),
        "f"  => Monosegment(consonant(&[vl, labiodental, sibilant, fricative], 'f')),
        "v"  => Monosegment(consonant(&[vd, labiodental, sibilant, fricative], 'v')),
        "θ"  => Monosegment(consonant(&[vl, dental, distrib, fricative], 'θ')),
        "ð"  => Monosegment(consonant(&[vd, dental, distrib, fricative], 'ð')),
        "s"  => Monosegment(consonant(&[vl, alveolar, sibilant, fricative], 's')),
        "z"  => Monosegment(consonant(&[vd, alveolar, sibilant, fricative], 'z')),
        "ʃ"  => Monosegment(consonant(&[vl, postalveolar, distrib, sibilant, fricative], 'ʃ')),
        "ʒ"  => Monosegment(consonant(&[vd, postalveolar, distrib, sibilant, fricative], 'ʒ')),
        "h"  => Monosegment(consonant(&[vl, glottal, fricative], 'h')),
        "l"  => Monosegment(consonant(&[vd, alveolar, lateral, distrib, approximant], 'l')),
        "ɹ"  => Monosegment(consonant(&[vd, alveolar, distrib, rhotic_c, approximant], 'ɹ')),
        "j"  => Monosegment(consonant(&[vd, palatal, glide], 'j')),
        "ʍ"  => Monosegment(consonant(&[vl, bilabial, velar, glide], 'ʍ')),
        "w"  => Monosegment(consonant(&[vd, bilabial, velar, glide], 'w')),

        // Vowels
        "i"  => Monosegment(vowel(&[high, front, tense], 'i')),
        "ɪ"  => Monosegment(vowel(&[high, front], 'ɪ')),
        "ɛ"  => Monosegment(vowel(&[mid, front, tense], 'ɛ')),
        "æ"  => Monosegment(vowel(&[mid, front], 'æ')),
        "ə"  => Monosegment(vowel(&[mid, front], 'ə')),
        "ʌ"  => Monosegment(vowel(&[mid, back], 'ʌ')),
        "ɑ"  => Monosegment(vowel(&[low, back], 'ɑ')),
        "u"  => Monosegment(vowel(&[high, back, rounded, tense], 'u')),
        "ʊ"  => Monosegment(vowel(&[high, back, rounded], 'ʊ')),
        "ɔ"  => Monosegment(vowel(&[mid, back, rounded], 'ɔ')),
        "e͡ɪ" =>   Disegment(vowel(&[mid, front, tense], 'e'),
                            vowel(&[high, front], 'ɪ')),
        "a͡ɪ" =>   Disegment(vowel(&[low, front], 'a'),
                            vowel(&[high, front], 'ɪ')),
        "a͡ʊ" =>   Disegment(vowel(&[low, front], 'a'),
                            vowel(&[high, back, rounded], 'ʊ')),
        "o͡ʊ" =>   Disegment(vowel(&[mid, back, tense, rounded], 'o'),
                            vowel(&[high, back, rounded], 'ʊ')),
        "ɔ͡ɪ" =>   Disegment(vowel(&[mid, back, rounded], 'ɔ'),
                            vowel(&[high, front], 'ɪ')),
        "ɜ˞" => Monosegment(vowel(&[mid, front, tense, rhotic_v], 'ɝ')),
        "ə˞" => Monosegment(vowel(&[mid, front, rhotic_v], 'ɚ')),
    };
}

fn consonant(fs: &[fn(&mut Segment)], sym: char) -> Segment {
    SegmentBuilder::consonant(fs, sym)
}

fn vowel(fs: &[fn(&mut Segment)], sym: char) -> Segment {
    SegmentBuilder::vowel(fs, sym)
}
