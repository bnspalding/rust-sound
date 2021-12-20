//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description
//! into a structured word (syllables and phonemes), using a given accent's phoneme
//! lookup function.

use crate::phoneme::Phoneme;
use crate::stress::Stress;
use crate::syllable::Syllable;
use crate::word::Word;

use std::error::Error;
use std::fmt;

/// from_accent constructs a word from a word description using an accent's
/// phoneme function.
///
/// from_accent(genam::phoneme, "ˈæ.pəl") -> Some(word for apple)
/// from_accent(genam::phoneme, "1ɑl.mo͡ʊst")
/// from_accent(genam::phoneme, "4əˈla͡ʊ")
///
/// The syntax used for word descriptions is meant to mirror (IPA) word pronunciation information
/// as it is given in a dictionary. Stress information may be given with IPA stress marks (ˈˌ.) or
/// numbers (1, 2, 3, 4) to mark stress levels and separate syllables. The absence of a stress mark
/// from the first syllable of a word is interpretted as unstressed (. or 3). Because there is no
/// IPA symbol for reduced stress, it must be marked using a number (4).
///
/// For syllabization to work, from_accent() expects exactly one -consonantal phoneme (a vowel) in
/// each syllable.
pub fn from_accent(
    accent: fn(&str) -> Option<Phoneme>,
    word_desc: &str,
) -> Result<Word, WordConstructorError> {
    let syls_as_symbols = split_word_desc(word_desc)?;

    // Construct each syllable and push to syls
    let mut syls = Vec::new();
    for mut syl_as_symbols in syls_as_symbols {
        // first symbol should be stress information
        let stress = match syl_as_symbols.remove(0).as_str() {
            "1" => Stress::Stressed,
            "2" => Stress::SecondaryStress,
            "3" => Stress::Unstressed,
            "4" => Stress::ReducedStress,
            _ => {
                return Err(WordConstructorError::new(
                    "BadSylStructure: no stress information for syllable",
                ))
            }
        };

        // Construct syllable from symbols
        let mut onset = vec![];
        let mut nucleus_maybe: Option<Phoneme> = None;
        let mut coda = vec![];
        for symbol in syl_as_symbols {
            // lookup phoneme for symbol or fail
            let phoneme = accent(&symbol).ok_or_else(|| {
                WordConstructorError::new(&format!(
                    "UnknownSymbol: {} not recognized in accent",
                    symbol
                ))
            })?;

            // vowels: only 1 vowel is permitted in a syllable
            if is_vowel(phoneme) {
                nucleus_maybe = match nucleus_maybe {
                    None => Ok(Some(phoneme)),
                    Some(existing_phoneme) => {
                        Err(WordConstructorError::new(&format!(
                            "BadSylStructure: two phonemes in syl: {}/{}",
                            existing_phoneme.symbol(),
                            symbol,
                        )))
                    }
                }?;
            //consonants: simply dependent on the vowel
            } else if nucleus_maybe.is_none() {
                onset.push(phoneme);
            } else {
                coda.push(phoneme);
            }
        } // for: end symbol iteration in syllable

        // ensure there was a vowel in the syllable
        let nucleus = nucleus_maybe.ok_or_else(|| {
            WordConstructorError::new("BadSylStructure: no nucleus in syllable")
        })?;

        syls.push(Syllable {
            onset,
            nucleus,
            coda,
            stress: Some(stress),
        });
    } // for: end syllable iteration

    Ok(syls.into())
}

fn split_word_desc(
    word_desc: &str,
) -> Result<Vec<Vec<String>>, WordConstructorError> {
    let mut symbol_iter = word_desc.chars().peekable();
    let mut syllables_as_symbols = Vec::new();
    let mut current_syllable: Vec<String> = Vec::new();

    // Enforce stress symbol on first syllable
    // If a stress symbol is already present, advance the iterator
    // Normalize stress symbol to (1..4)
    let first_stress_symbol = symbol_iter
        .peek()
        .copied()
        .map(|c| {
            if "1234".contains(c) {
                symbol_iter.next();
                c
            } else if "ˈˌ.".contains(c) {
                symbol_iter.next();
                match c {
                    'ˈ' => '1',
                    'ˌ' => '2',
                    _ => '3',
                }
            } else {
                '3'
            }
        })
        .ok_or_else(|| {
            WordConstructorError::new("BadSylStructure: empty string")
        })?;
    current_syllable.push(String::from(first_stress_symbol));

    while let Some(current) = symbol_iter.next() {
        let mut current_symbol = String::from(current);

        // stress symbols
        let mut new_syllable_flag = false;
        //normalize stress symbols to (1..4)
        if "ˈˌ.".contains(current_symbol.as_str()) {
            new_syllable_flag = true;
            current_symbol = String::from(match current_symbol.as_str() {
                "ˈ" => "1",
                "ˌ" => "2",
                _ => "3",
            })
        } else if "1234".contains(current_symbol.as_str()) {
            new_syllable_flag = true;
        }
        // shift to a new syllable when a stress symbol is encountered
        if new_syllable_flag {
            syllables_as_symbols.push(current_syllable);
            current_syllable = Vec::new();
        }

        // multi-character symbols
        match symbol_iter.peek().copied() {
            // u/0361 connector symbol (a͡ʊ)
            Some('\u{0361}') => {
                let connector = symbol_iter.next().unwrap();
                let connected_symbol = symbol_iter.next().ok_or_else(|| {
                    WordConstructorError::new(
                        "BadWordDesc: connector u/0361 given without following symbol")})?;
                current_symbol.push(connector);
                current_symbol.push(connected_symbol);
            }
            //rhotic symbol (ə˞)
            Some('\u{02DE}') => {
                let rhotic_symbol = symbol_iter.next().unwrap();
                current_symbol.push(rhotic_symbol);
            }
            _ => {}
        }

        current_syllable.push(current_symbol);
    } // while: end symbol iteration

    Ok(syllables_as_symbols)
}

fn is_vowel(p: Phoneme) -> bool {
    //NOTE: This code should live somewhere else. It should be easy to check if a phoneme
    //belongs to some basic classes
    use crate::features::BinaryFeature;

    match p {
        Phoneme::Monosegment(seg) => {
            seg.root_features.syllabic == BinaryFeature::Marked
        }
        Phoneme::Disegment(seg1, seg2) => {
            seg1.root_features.syllabic == BinaryFeature::Marked
                || seg2.root_features.syllabic == BinaryFeature::Marked
        }
    }
}

/// An error created during the construction of a word from a word description string
//TODO: give this error a clearer enum structure
// Types of errors:
//   - unknown phoneme (return None from accent fn)
//   - bad syllable structure
//      - no vowel (no nucleus),
//      - too many vowels,
//      - empty syllable (only stress mark)
//      - no stress information for syllable
//  - bad word description
#[derive(Debug)]
pub struct WordConstructorError {
    msg: String,
}

impl WordConstructorError {
    fn new(msg: &str) -> WordConstructorError {
        WordConstructorError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for WordConstructorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error for WordConstructorError {
    fn description(&self) -> &str {
        &self.msg
    }
}
