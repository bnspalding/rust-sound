//! Levels of lexical stress for syllables
//!
//! Stress provides four levels of marked stress for a syllable. The four
//! levels of stress can also reduce to either high or low stress for scenarios
//! where a binary measure of stress is more appropriate.
//!
//! There doesn't appear to be solid agreement about how many levels of
//! distinguishable stress are useful, so the choice to use four (with means
//! for reducing to two) is motivated by leaving the greatest number of options
//! open. The CMU Pronouncing Dictionary uses three levels, which would map to
//! 0-Unstressed, 1-Stressed 2-SecondaryStress.

/// Stress is represented with four levels of emphasis. Use [`toBinaryStress`]
/// to reduce these four levels to binary stress.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum Stress {
    /// The syllable is not only least emphasized, but also reduced
    ReducedStress,
    /// The syllable is less emphasized than surrounding syllables
    Unstressed,
    /// The syllable has stress, but is not the most prominent in a word
    SecondaryStress,
    /// The syllable is more emphasized than surrounding syllables
    Stressed,
}

impl Stress {
    /// to_binary_stress converts four-level stress to two-level stress.
    ///
    /// ReducedStress and Unstressed become Unstressed,
    /// SecondaryStress and Stressed become Stressed.
    pub fn to_binary_stress(self) -> BinaryStress {
        match self {
            Stress::ReducedStress | Stress::Unstressed => {
                BinaryStress::Unstressed
            }
            Stress::SecondaryStress | Stress::Stressed => {
                BinaryStress::Stressed
            }
        }
    }

    /// symbol provides the IPA symbol associated with the stress level, if a
    /// symbol is associated with that level of stress.
    ///
    /// ReducedStress and Unstressed levels have no marker.
    /// SecondaryStress is marked with 'ˌ'.
    /// Stressed is marked with 'ˈ'.
    pub fn symbol(&self) -> Option<char> {
        match self {
            Stress::ReducedStress | Stress::Unstressed => None,
            Stress::SecondaryStress => Some('ˌ'),
            Stress::Stressed => Some('ˈ'),
        }
    }
}

/// BinaryStress represents two levels of syllable emphasis
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub enum BinaryStress {
    /// The syllable is less emphasized than surrounding syllables
    Unstressed,
    /// The syllable is more emphasized than surrounding syllables
    Stressed,
}
