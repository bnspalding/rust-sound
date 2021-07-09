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
//!  [round]  [+/-anterior][+/-distib]  [+/-high][+/-low][+/-back]  [+/-ATR]
//!     |                |    |                  \    |   /             |
//!  [labial]           [coronal]                 [dorsal]        [pharyngeal]
//!      \__________________|_________________________|________________/
//!                                      |
//!                                   [place]
//!                                      |
//!                                  X SEGMENT
//!                              (+/- consonantal)
//!                               (+/- sonorant)
//!                               (+/- syllabic)
//!        ______________________________|____________________________
//!       /               |            |         |         |          \
//![+/-continuant]  [+/-strident]  [lateral]  [nasal]  [laryngeal]  [rhotic]
//!                                                    /    |    \
//!                                                  [SG]  [CG]  [+/-voice]
//!</pre>

///A Binary Feature describes a contrastive feature.
///
///Both the markedness (+) or unmarkedness (-) of the feature can be used to
///construct a natural class of sounds. This is different from Unary Features,
///such as Nasal or Lateral, which are only meaningful as marked classes
///(-nasal is not a utilized class of sounds).
///
///When a feature is absent (neither marked or unmarked), it means that the
///mechanical preconditions for the feature are not present. For example, vowel
///space features such as [+/-high], [+/-low], [+/-back] are not specified for
///dorsal consonants.
pub enum BinaryFeature {
    ///The feature contrasts positively (it is notably there).
    Marked,
    ///The feature contrasts negatively (it is notably not there).
    Unmarked,
}

///A Unary Feature is meaningful only when marked.
///
///Natural classes do not form around the lack of a Unary Feature. When a
///feature is absent, it means that the mechanical preconditions
///for the feature are not present. For example, \[nasal\] is not specified
///for non-nasal consonants and non-nasal consonants do not form a useful
///natural class in sound categorization.
pub enum UnaryFeature {
    ///The feature is notably present on the segment
    Marked,
}

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
    pub autosegmental_features: AutosegmentalFeatures,
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
    ///Presence at the nucleus of a syllable: vowels (+); consonants, glides (-).
    pub syllabic: BinaryFeature,
}

///Autosegmental Features describe phonological segments in a variety of ways.
///
///These features can be targetted during transformations at a scope beyond
///individual segments (autonomously), hence the descriptor 'autosegmental'.
///
///Some features are dependent on the presence of other parent features,
///resulting in a tree structure.
pub struct AutosegmentalFeatures {
    ///air passes through the nasal tract: 'n', 'm', 'ŋ'.
    pub nasal: Option<UnaryFeature>,
    ///air passes to sides around tongue: 'l', 'ɹ'.
    pub lateral: Option<UnaryFeature>,
    ///any of the different ways that rhoticity is marked.
    pub rhotic: Option<UnaryFeature>,
    ///high-amplitude, high frequency fricatives: sibilants (+).
    pub strident: Option<BinaryFeature>,
    ///continuous vs stopped air flow: fricatives, approximants (+), stops (-).
    pub continuant: Option<BinaryFeature>,
    ///place of articulation within the mouth.
    pub place: Option<Place>,
    ///contrasts and distinctions at the larynx: voicing distinctions.
    pub laryngeal: Option<LaryngealFeatures>,
}

///Place describes a location of constriction/articulation within the mouth.
///
///This feature group captures dependencies of features that only appear at
///certain points of articulation in the mouth. It also permits transformations
///to target place of articulation as group of features.
pub struct Place {
    ///articulation using the lips: 'p', 'm', vowel rounding.
    pub labial: Option<LabialFeature>,
    ///articulation using the front of the tongue: 't', 's', 'n'.
    pub coronal: Option<CoronalFeature>,
    ///articulation using the body of the tongue: 'k', 'ŋ', vowel space.
    pub dorsal: Option<DorsalFeature>,
    ///articulation using the root of the tongue: ATR.
    pub pharyngeal: Option<PharyngealFeature>,
}

///Features determined by behavior involving the lips.
pub struct LabialFeature {
    ///rounding of the lips during sound production: round vowels (+).
    pub round: Option<UnaryFeature>,
}

///Features determined by behavior involving the front of the tongue.
pub struct CoronalFeature {
    ///relation of the tongue to the alveolar ridge: dentals, alveolars (+).
    pub anterior: Option<BinaryFeature>,
    ///tongue blade (laminal) vs tongue tip (apical): 'ʃ', 'θ' (+); 's' (-).
    pub distrib: Option<BinaryFeature>,
}

///Features determined by behavior involving the body of the tongue.
///
///Vowel space is defined with both a [+/-high] and a [+/-low], following a
///tradition of characterizing high vowels as (+high,-low), low vowels as
///(-high, +low), and mid vowels as (-high, -low).
pub struct DorsalFeature {
    ///high tongue position: high vowels (+); mid and low vowels (-).
    pub high: Option<BinaryFeature>,
    ///low tongue position: low vowels (+); mid and high vowels (-).
    pub low: Option<BinaryFeature>,
    ///tongue is not front: back and central vowels (+); front vowels (-).
    pub back: Option<BinaryFeature>,
}

///Features determined by behavior at the root of the tongue.
pub struct PharyngealFeature {
    ///tongue root is forward. doubles as [+/-tense]. 'i', 'e', 'u', 'o' (+).
    ///ATR should be undefined for low vowels.
    pub advanced_tongue_root: Option<BinaryFeature>,
}

///Features determined by the behavior of the vocal folds.
pub struct LaryngealFeatures {
    ///open vocal folds: aspirated segments.
    pub spread_glottis: Option<UnaryFeature>,
    ///constricted vocal folds: ejectives, glottal stops.
    pub constricted_glottis: Option<UnaryFeature>,
    ///vibrating vocal folds: 'b', 'd', 'ɡ' (+); 'p', 't', 'k' (-)
    pub voice: Option<BinaryFeature>,
}
