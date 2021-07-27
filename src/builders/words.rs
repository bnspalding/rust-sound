//! Constructor for words
//!
//! Builders::Words provides a constructor for words. It parses a word description
//! into a structured word (syllables and phonemes), using a given accent's phoneme
//! lookup function.

use crate::phoneme::Phoneme;
use crate::word::Word;

use std::error::Error;
use std::fmt;

/// from_accent constructs a word from a word description using an accent's
/// phoneme function.
///
/// from_accent(genam::phoneme, "ˈæ.p/ə/l") -> Some(word for apple)
/// from_accent(genam::phoneme, "ˈæ.(p/ə/l)")
/// from_accent(genam::phoneme, "1/ɑ/l.m/o͡ʊ/st")
/// from_accent(genam::phoneme, "(1/ɑ/l).(m/o͡ʊ/st)")
/// from_accent(genam::phoneme, "4əˈl/a͡ʊ/")
/// from_accent(genam::phoneme, "4ə1l/a͡ʊ/")
///
/// use "/" to mark onset vs nucleus vs coda. can be ommitted for nucleus only (single phoneme)
/// use IPA stress marks(ˈˌ.4) or numbers (1,2,3,4) to mark stress levels and separate syllables
/// stress mark goes at the beginning of the syllable, assume 3 for first syllable when ommitted
/// parentheses are used to mark syllables for a reader, stripped during parsing
pub fn from_accent<F>(
    accent: F,
    word_desc: &str,
) -> Result<Word, WordConstructorError>
where
    F: FnOnce(&str) -> Option<Phoneme>,
{
    todo!()
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
