//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description
//! into a structured word (syllables and phonemes), using a given accent's phoneme
//! lookup function.

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
// Because +/-syllabic is a feature, slashes can hypothetically be ommitted.
// ˈæ.pəl is fine on its own because you can just place a single syllabic phoneme into the nucleus
// This works so long as the assumption that all syllables contain a single syllabic item is held
//
// Reduced stress is still a thing not specified by standard IPA, so supporting an alternate
// notation for stress (1-4) makes sense.
pub fn from_accent<F>(
    accent: F,
    word_desc: &str,
) -> Result<Word, WordConstructorError>
where
    F: Fn(&str) -> Option<Phoneme>,
{
    let mut proto_word = vec![];

    // TODO IMPORTANT: I haven't verified that grapheme segmentation actually segments correctly for IPA
    // symbols. For example, is a stress mark (such as 'ˈ') its own segment? are two joined letters
    // (t͡ʃ) a single segment?
    let mut graphemes =
        UnicodeSegmentation::graphemes(word_desc, true).peekable();

    //Special handling: peek first symbol for optional stress/syllable mark
    match graphemes.peek() {
        None => {
            return Err(WordConstructorError::new("word description is empty"));
        }
        Some(first_symbol) => if !is_stress_symbol(first_symbol) {},
    };
    //Parse remaining symbols in word description
    for symbol in graphemes {
        if let Some(err) = read_symbol(symbol, &mut proto_word, &accent) {
            return Err(err);
        }
    }

    //convert the constructed protoWord into a word
    //If it is only one syllable long, remove stress information
    let word = Word {
        syls: proto_word
            .iter()
            .map(|syl| Syllable {
                onset: syl.onset.clone(),
                nucleus: syl.nucleus.unwrap(),
                coda: syl.coda.clone(),
                stress: if proto_word.len() == 1 {
                    None
                } else {
                    syl.stress
                },
            })
            .collect(),
    };

    Ok(word)
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
    None
}

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
