//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description into a structured word (syllables and phonemes), using a given accent's phoneme lookup function.

use crate::feature_classes;
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
///
/// Because lexical stress is only useful as a comparison between syllables of the same word,
/// single syllable words should have None as their stress information.
pub fn from_accent(
    accent: fn(&str) -> Option<Phoneme>,
    word_desc: &str,
) -> Result<Word, WordConstructorError> {
    let syls_as_symbols = split_word_desc(word_desc)?;

    let multiple_syllables_flag = syls_as_symbols.len() > 1;

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
            if feature_classes::is_vowel(phoneme) {
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
            stress: if multiple_syllables_flag {
                Some(stress)
            } else {
                None
            },
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

    //push final syllable
    syllables_as_symbols.push(current_syllable);

    Ok(syllables_as_symbols)
}

/// An error created during the construction of a word from a word description string
///
/// For the moment, these errors are contained by a single type and differentied only in their
/// description. The resolution to all of the errors is "examine your input string for errors", so
/// it doesn't feel sensible to differentiate the errors further than by providing some guidance in
/// the message.
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

impl Error for WordConstructorError {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::features;
    use crate::features::Segment;

    //mock accent used for testing
    fn mock_accent(s: &str) -> Option<Phoneme> {
        // when adding new symbols, be sure to also mark vowels as syllabic in mock_seg
        match s {
            "h" => Some(mock_phon_m('h')),
            "ɛ" => Some(mock_phon_m('ɛ')),
            "l" => Some(mock_phon_m('l')),
            "o͡ʊ" => Some(mock_phon_d('o', 'ʊ')),
            "t" => Some(mock_phon_m('t')),
            "s" => Some(mock_phon_m('s')),
            "i" => Some(mock_phon_m('i')),
            "ə˞" => Some(mock_phon_m('ɚ')),
            _ => None,
        }
    }

    fn mock_phon_m(sym: char) -> Phoneme {
        Phoneme::Monosegment(mock_seg(sym))
    }

    fn mock_phon_d(sym1: char, sym2: char) -> Phoneme {
        Phoneme::Disegment(mock_seg(sym1), mock_seg(sym2))
    }

    fn mock_seg(sym: char) -> Segment {
        // The only fields that matter here are symbol and root_features.syllabic
        Segment {
            symbol: sym,
            root_features: features::RootFeatures {
                // mark vowels as +syllabic
                syllabic: if "ɛɚʊoi".contains(sym) {
                    features::BinaryFeature::Marked
                } else {
                    features::BinaryFeature::Unmarked
                },
                sonorant: features::BinaryFeature::Unmarked,
                consonantal: features::BinaryFeature::Unmarked,
            },
            autosegmental_features: features::AutosegmentalFeatures::default(),
        }
    }

    #[test]
    //testing multiple syllables, breve-connected symbols, syllable break marker, only onsets
    fn test_from_accent() -> Result<(), WordConstructorError> {
        println!("{:?}", split_word_desc("ˈhɛ.lo͡ʊ"));
        assert_eq!(
            from_accent(mock_accent, "ˈhɛ.lo͡ʊ")?,
            Word::from(vec![
                Syllable {
                    onset: vec![mock_phon_m('h')],
                    nucleus: mock_phon_m('ɛ'),
                    coda: vec![],
                    stress: Some(Stress::Stressed),
                },
                Syllable {
                    onset: vec![mock_phon_m('l')],
                    nucleus: mock_phon_d('o', 'ʊ'),
                    coda: vec![],
                    stress: Some(Stress::Unstressed),
                }
            ])
        );
        Ok(())
    }

    #[test]
    //testing single syl, no stress, both onset and codas
    fn test_from_accent_single_syl() -> Result<(), WordConstructorError> {
        println!("{:?}", split_word_desc("tɛst"));
        assert_eq!(
            from_accent(mock_accent, "tɛst")?,
            Word::from(vec![Syllable {
                onset: vec![mock_phon_m('t')],
                nucleus: mock_phon_m('ɛ'),
                coda: vec![mock_phon_m('s'), mock_phon_m('t')],
                stress: None,
            }])
        );
        Ok(())
    }

    #[test]
    //testing use of numbered stress
    fn test_from_accent_numbered_stress() -> Result<(), WordConstructorError> {
        println!("{:?}", split_word_desc("2ti4tə˞"));
        assert_eq!(
            from_accent(mock_accent, "2ti4tə˞")?,
            Word::from(vec![
                Syllable {
                    onset: vec![mock_phon_m('t')],
                    nucleus: mock_phon_m('i'),
                    coda: vec![],
                    stress: Some(Stress::SecondaryStress),
                },
                Syllable {
                    onset: vec![mock_phon_m('t')],
                    nucleus: mock_phon_m('ɚ'),
                    coda: vec![],
                    stress: Some(Stress::ReducedStress),
                }
            ])
        );
        Ok(())
    }
}
