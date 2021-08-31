//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description
//! into a structured word (syllables and phonemes), using a given accent's phoneme
//! lookup function.

use crate::phoneme::Phoneme;
use crate::word::Word;
use crate::syllable::Syllable;
use crate::stress::Stress;

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
//
// from_accent(genam::phoneme, "ˈæ.p/ə/l") -> Some(word for apple)
// from_accent(genam::phoneme, "ˈæ.(p/ə/l)")
// from_accent(genam::phoneme, "1/ɑ/l.m/o͡ʊ/st")
// from_accent(genam::phoneme, "(1/ɑ/l).(m/o͡ʊ/st)")
// from_accent(genam::phoneme, "4əˈl/a͡ʊ/")
// from_accent(genam::phoneme, "4ə1l/a͡ʊ/")
//
// use "/" to mark onset vs nucleus vs coda. can be ommitted for nucleus only (single phoneme)
// use IPA stress marks(ˈˌ.4) or numbers (1,2,3,4) to mark stress levels and separate syllables
// stress mark goes at the beginning of the syllable, assume 3 for first syllable when ommitted
// parentheses are used to mark syllables for a reader, stripped during parsing
//
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
    let mut graphemes = UnicodeSegmentation::graphemes(word_desc, true);

    //Special handling: first syllable
    match graphemes.next() {
        None => {return Err(WordConstructorError::new("word description is empty"));}
        Some(first_symbol) => {
            if !is_stress_symbol(first_symbol) {
                let mut syl = ProtoSyl::new();
                syl.stress = Some(Stress::Unstressed);
                proto_word.push(syl);
            }
            read_symbol(first_symbol, &mut proto_word, &accent);
        }
    };
    //Parse remaining symbols in word description
    for symbol in graphemes {
        if let Some(err) = read_symbol(symbol, &mut proto_word, &accent) {
            return Err(err)
        }
    }

    //convert the constructed protoWord into a word
    //If it is only one syllable long, remove stress information
    let word = todo!();

    Ok(word)
}

// Read a symbol from a word_desc and modify the under-construction word accordingly
fn read_symbol<F>(
    symbol: &str, 
    proto_word: &mut ProtoWord, 
    accent: F
    ) -> Option<WordConstructorError>
where
    F: Fn(&str) -> Option<Phoneme> {
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
             _  => Stress::Unstressed,
        });
        proto_word.push(syl);
        return None;
    }

    match accent(symbol) {
        None => { return Some(WordConstructorError::no_phoneme(symbol));}
        Some(phoneme) => {
            //2. +syllabic phoneme: add to nucleus, flip from onset additions to coda additions
            //3. other phoneme: add to either onset or coda, depending on nucleus status
            todo!()
        }
    };

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
