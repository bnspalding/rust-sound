//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description into a structured word (syllables and phonemes), using a given accent's phoneme lookup function.

use crate::features;
use crate::phoneme::Phoneme;
use crate::stress::Stress;
use crate::syllable::Syllable;
use crate::word::Word;

use std::error::Error;
use std::fmt;
use unicode_segmentation::UnicodeSegmentation;

// ProtoSyl is used during construction to hold the components of a syllable. This is necessary
// because the nucleus is undefined until construction reaches it.
struct ProtoSyl {
    onset: Vec<Phoneme>,
    nucleus: Option<Phoneme>,
    coda: Vec<Phoneme>,
    stress: Option<Stress>,
}

impl ProtoSyl {
    fn new() -> ProtoSyl {
        ProtoSyl {
            onset: vec![],
            nucleus: None,
            coda: vec![],
            stress: None,
        }
    }
}

type ProtoWord = Vec<ProtoSyl>;

/// from_accent constructs a word from a word description using an accent's
/// phoneme function.
///
/// Stress can be written using either IPA notation (ˈ, ˌ, .) or numbers (1,2,3,4). The only way to
/// note reduced stress on a syllable is to use the number 4, because there is no corresponding
/// symbol in IPA.
///
/// Special consideration has been made for combining breve (u0361) and combining r hook (u02de) so
/// that these symbols can be used as part of word descriptions.
pub fn from_accent<F>(
    accent: F,
    word_desc: &str,
) -> Result<Word, WordConstructorError>
where
    F: Fn(&str) -> Option<Phoneme>,
{
    let mut proto_word = vec![];

    let mut graphemes =
        UnicodeSegmentation::graphemes(word_desc, true).peekable();

    //Parse remaining symbols in word description
    //We need to use while instead of a standard for...in in order to handle multi-grapheme symbols
    //that use the combineing breve (u0361). Grapheme segmentation combines the breve with the
    //frist symbol of the sequence, but reads the second symbol as an independent grapheme.
    while let Some(symbol) = graphemes.next() {
        //special handling for combining breve (u0361): get the next symbol and combine
        let mut symbol_checked = if symbol.contains('͡') {
            symbol.to_owned() + graphemes.next().unwrap()
        } else {
            symbol.to_string()
        };

        //special handling for following r hook (u02DE): combine it with the current symbol
        if let Some(&"˞") = graphemes.peek() {
            symbol_checked += graphemes.next().unwrap()
        }

        if let Some(err) =
            read_symbol(&symbol_checked, &mut proto_word, &accent)
        {
            return Err(err);
        }
    }

    // Build a word from the proto_word.
    // If it is only one syllable long, remove stress information
    let checked_syls = proto_word
        .iter()
        .map(|syl| {
            if syl.nucleus.is_none() {
                Err(WordConstructorError::new(
                    "Bad structure: no nucleus in syllable",
                ))
            } else {
                Ok(Syllable {
                    onset: syl.onset.clone(),
                    nucleus: syl.nucleus.unwrap(), // This has been checked above. Should be okay.
                    coda: syl.coda.clone(),
                    stress: if proto_word.len() == 1 {
                        None
                    } else {
                        syl.stress
                    },
                })
            }
        })
        .collect();

    // I'm sure this can be simplified, but I don't know how yet
    match checked_syls {
        Ok(syls) => Ok(Word { syls }),
        Err(err) => Err(err),
    }
}

// Read a symbol from a word_desc and modify the under-construction word accordingly
fn read_symbol<F>(
    symbol: &str,
    proto_word: &mut ProtoWord,
    accent: F,
) -> Option<WordConstructorError>
where
    F: Fn(&str) -> Option<Phoneme>,
{
    // Handle stress mark or syllable break
    if is_stress_symbol(symbol) {
        let mut syl = ProtoSyl::new();
        syl.stress = Some(match symbol {
            "1" => Stress::Stressed,
            "2" => Stress::SecondaryStress,
            "3" => Stress::Unstressed,
            "4" => Stress::ReducedStress,
            "ˈ" => Stress::Stressed,
            "ˌ" => Stress::SecondaryStress,
            _ => Stress::Unstressed,
        });
        proto_word.push(syl);
        return None;
    }

    // Lookup phoneme associated with symbol
    let phoneme = match accent(symbol) {
        None => {
            return Some(WordConstructorError::no_phoneme(symbol));
        }
        Some(p) => p,
    };

    // Get the current syllable. If there are no syllables (for example, from no optional stress
    // marker at the start of the word_desc) create an unstressed syllable.
    let mut current_syl = match proto_word.last_mut() {
        Some(s) => s,
        None => {
            let mut syl = ProtoSyl::new();
            syl.stress = Some(Stress::Unstressed);
            proto_word.push(syl);
            proto_word.last_mut().unwrap()
        }
    };

    // TODO: write a better way of querying a syllable
    // Check if phoneme is +syllabic
    let is_syllabic = match phoneme {
        Phoneme::Monosegment(seg) => {
            seg.root_features.syllabic == features::BinaryFeature::Marked
        }
        Phoneme::Disegment(seg1, seg2) => {
            seg1.root_features.syllabic == features::BinaryFeature::Marked
                || seg2.root_features.syllabic
                    == features::BinaryFeature::Marked
        }
    };

    // syllabic phonemes go in the nucleus
    if is_syllabic {
        match current_syl.nucleus {
            None => {
                current_syl.nucleus = Some(phoneme);
            }
            Some(p) => {
                return Some(WordConstructorError::new(&format!(
                    "{}: nucleus is already occupied by {}",
                    phoneme.symbol(),
                    p.symbol()
                )))
            }
        }
        return None; // exit successfully
    }

    // non syllabic phonemes go to either the onset or coda, depending whether or not we've reached
    // the nucleus of the syllable
    if current_syl.nucleus.is_none() {
        current_syl.onset.push(phoneme);
    } else {
        current_syl.coda.push(phoneme);
    }

    None // complete with no errors
}

// The collection of stress/syllable break symbols used in word builder descriptions
fn is_stress_symbol(sym: &str) -> bool {
    "1234ˈˌ.".contains(sym)
}

/// An error created during the construction of a word from a word description string
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

    fn no_phoneme(symbol: &str) -> WordConstructorError {
        WordConstructorError {
            msg: format!("unknown symbol {} has no phoneme", symbol),
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

#[cfg(test)]
mod tests {

    use super::*;
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
        assert_eq!(
            from_accent(mock_accent, "ˈhɛ.lo͡ʊ")?,
            Word {
                syls: vec![
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
                ]
            }
        );
        Ok(())
    }

    #[test]
    //testing single syl, no stress, both onset and codas
    fn test_from_accent_single_syl() -> Result<(), WordConstructorError> {
        assert_eq!(
            from_accent(mock_accent, "tɛst")?,
            Word {
                syls: vec![Syllable {
                    onset: vec![mock_phon_m('t')],
                    nucleus: mock_phon_m('ɛ'),
                    coda: vec![mock_phon_m('s'), mock_phon_m('t')],
                    stress: None,
                }]
            }
        );
        Ok(())
    }

    #[test]
    //testing use of numbered stress
    fn test_from_accent_numbered_stress() -> Result<(), WordConstructorError> {
        assert_eq!(
            from_accent(mock_accent, "2ti4tə˞")?,
            Word {
                syls: vec![
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
                ]
            }
        );
        Ok(())
    }
}
