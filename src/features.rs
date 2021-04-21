//! Distinctive features for describing phonemes.
//!
//! ## Distinctive Features
//! -- what is a feature (as an abstraction)
//! -- binary and unary features
//! -- what assumptions/theories am I using?
//! -- link to wiki pages/theory descriptions
//!
//!
//! ## Autosegmental Features
//!
//!
//! ## Feature Geometry
//! -- write an explanation of feature geometry (what and why)
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
///[`Phoneme`]: ../phoneme/enum.Phoneme.html
///[`Root Features`]: struct.RootFeatures.html
///[`Autosegmental Features`]: struct.AutosegmentalFeatures.html
pub struct Segment {
    ///root features on a segment
    pub root_features: RootFeatures,
    ///autosegmental features on a segment
    pub autosegmental_features: HashSet<AutosegmentalFeatures>,
    ///symbolic representation of a segment
    pub symbol: String,
}

///Root Features describe all phonological segments.
///
///These features are bound to a segment and do not exhibit autosegmental
///behaviors.
pub struct RootFeatures {
    ///Constriction of the vocal tract: consonants (+); vowels (-).
    pub consonantal: BinaryFeature,
    ///Resonant vs turbulent sound: nasals, liquids, vowels (+); obstruents (-).
    pub sonorant: BinaryFeature,
}

///Autosegmental Features describe phonological segments in a variety of ways.
///
///These features can be targetted during transformations at a scope beyond
///individual segments (autonomously), hence the descriptor 'autosegmental'.
///
///Some features are dependent on the presence of other parent features,
///resulting in a tree structure.
pub enum AutosegmentalFeatures {
    ///air passes through the nasal tract: 'n', 'm', 'ŋ'.
    Nasal,
    ///air passes to sides around tongue: 'l', 'ɹ'.
    Lateral,
    ///high-amplitude, high frequency fricatives: sibilants (+).
    Strident(BinaryFeature),
    ///continuous vs stopped air flow: fricatives, approximants (+), stops (-).
    Continuant(BinaryFeature),
    ///place of articulation within the mouth.
    Place(HashSet<PlaceNode>),
    ///contrasts and distinctions at the larynx: voicing distinctions.
    Laryngeal(HashSet<LaryngealFeature>),
}

///PLACE describes the point of constriction/articulation within the mouth.
///
///PLACE captures dependencies of features that only appear at certain points of
///articulation in the mouth. It also permits transformations to target place of
///articulation as group of features.
///
///A PLACE node with an empty set of child features is still meaningful, and
///describes the place of articulation without any further features specified.
pub enum PlaceNode {
    ///articulation using the lips: 'p', 'm', vowel rounding.
    Labial(HashSet<LabialFeature>),
    ///articulation using the front of the tongue: 't', 's', 'n'.
    Coronal(HashSet<CoronalFeature>),
    ///articulation using the body of the tongue: 'k', 'ŋ', vowel space.
    Dorsal(HashSet<DorsalFeature>),
    ///articulation using the root of the tongue: ATR.
    Pharyngeal(HashSet<PharyngealFeature>),
}

///Features determined by behavior involving the lips
pub enum LabialFeature {
    ///vowel rounding
    Round,
}

///Features determined by behavior involving the front of the tongue
pub enum CoronalFeature {
    ///relation of the tongue to the alveolar ridge: dentals, alveolars (+).
    Anterior(BinaryFeature),
    ///tongue blade (laminal) vs tongue tip (apical): 'ʃ', 'θ' (+); 's' (-).
    Distrib(BinaryFeature),
}

///Features determined by behavior involving the body of the tongue
///
///Vowel space is defined with both a [+/-high] and a [+/-low], following a
///tradition of characterizing high vowels as (+high,-low), low vowels as
///(-high, +low), and mid vowels as (-high, -low).
pub enum DorsalFeature {
    ///high tongue position: high vowels (+); mid and low vowels (-).
    High(BinaryFeature),
    ///low tongue position: low vowels (+); mid and high vowels (-).
    Low(BinaryFeature),
    ///tongue is not front: back and central vowels (+); front vowels (-).
    Back(BinaryFeature),
}

///Features determined by behavior at the root of the tongue
pub enum PharyngealFeature {
    ///tongue root is forward. doubles as [+/-tense]. 'i', 'e', 'u', 'o' (+).
    ///ATR should be undefined for low vowels.
    AdvancedTongueRoot(BinaryFeature),
}

///Features determined by the behavior of the vocal folds.
pub enum LaryngealFeature {
    ///open vocal folds: aspirated segments.
    SpreadGlottis,
    ///constricted vocal folds: ejectives, glottal stops.
    ConstrictedGlottis,
    ///vibrating vocal folds: 'b', 'd', 'ɡ' (+); 'p', 't', 'k' (-)
    Voice(BinaryFeature),
}

///A Binary Feature describes a contrastive feature.
///
///Both the markedness (+) or unmarkedness (-) of the feature can be used to
///construct a natural class of sounds. This is different from Unary Features,
///such as Nasal or Lateral, which are only meaningful as marked classes (-nasal
///is not a utilized class of sounds).
///
///When a feature is absent (neither marked or unmarked), it means that the
///mechanical preconditions for the feature are not present. For example, vowel
///space features such as [+/-high], [+/-low], [+/-back] are not specified for
///dorsal consonants.
pub enum BinaryFeature {
    ///The feature contrasts positively (it is notably there).
    Plus,
    ///The feature contrasts negatively (it is notably not there).
    Minus,
}
