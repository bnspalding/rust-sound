//! TODO: write module documentation
//! -- what is a feature (as an abstraction)
//! -- what assumptions/theories am I using?
//! -- link to wiki pages/theory descriptions
//!
//! ## Feature Geometry Diagram
//!<pre>
//![+/-round]  [+/-anterior][+/-distib]  [+/-high][+/-low][+/-back]  [+/-ATR]
//!     |                |    |                  \    |   /             |
//!  [labial]           [coronal]                 [dorsal]        [pharyngeal]
//!      \__________________|_________________________|________________/
//!                                      |
//!                                    PLACE
//!                                      |
//!                                  X segment
//!                              (+/- consonantal)
//!                               (+/- sonorant)
//!       _______________________________|________________________
//!       |                  |           |           |           |
//![+/-continuant]    [+/-strident]   [lateral]   [nasal]   [laryngeal]
//!                                                         /    |    \
//!                                                       [SG]  [CG]  [+/-voice]
//!</pre>

use std::collections::HashSet;

///A Segment is a structured collection of phonological features used to
///describe a [`Phoneme`].
///
///All segments have a collection of [`Root Features`] that are bound to the
///segment. [`Autosegmental Features`] are more fluid, and only a subset of all
///autosegmental features are specified for any segment. These features behave
///differently from root features when a segment undergoes a phonological
///transformation.
///
///[`Phoneme`]: phoneme.struct.Phoneme.html
///[`Root Features`]: struct.RootFeatures.html
///[`Autosegmental Features`]: struct.AutosegmentalFeatures.html
pub struct Segment {
    ///root features on a segment
    pub root_features: RootFeatures,
    ///autosegmental features on a segment
    pub autosegmental_features: HashSet<AutosegmentalFeatures>,
}

///Root Features describe all phonological segments.
///
///These features are bound to a segment and do not exhibit autosegmental
///behaviors.
pub struct RootFeatures {
    ///Distinguishing consonants and vowels
    pub consonantal: BinaryFeature,
    ///Distinguishing resonant sounds and obstruents.
    pub sonorant: BinaryFeature,
}

///Autosegmental Features describe phonological segments in a variety of ways.
///
///These features all exhibit behaviors during transformations that scope beyond
///individual segments (autonomous behavior), hence the descriptor 'autosegmental'.
///
///Some features are dependent on the presence of other parent features,
///resulting in a tree structure that looks like:

pub enum AutosegmentalFeatures {
    Nasal,
    Lateral,
    Strident(BinaryFeature),
    Continuant(BinaryFeature),
    Place(PlaceNode),
    Laryngeal(HashSet<LaryngealFeature>),
}

pub enum PlaceNode {
    Labial(HashSet<LabialFeature>),
    Coronal(HashSet<CoronalFeature>),
    Dorsal(HashSet<DorsalFeature>),
    Pharyngeal(HashSet<PharyngealFeature>),
}

pub enum LabialFeature {
    Round(BinaryFeature),
}

pub enum CoronalFeature {
    Anterior(BinaryFeature),
    Distrib(BinaryFeature),
}

pub enum DorsalFeature {
    High(BinaryFeature),
    Low(BinaryFeature),
    Back(BinaryFeature),
}

pub enum PharyngealFeature {
    AdvancedTongueRoot(BinaryFeature),
}

pub enum LaryngealFeature {
    SpreadGlottis,
    ConstrictedGlottis,
    Voice(BinaryFeature),
}

pub enum BinaryFeature {
    Plus,
    Minus,
}
