//! Distinctive features for describing phonemes.
//!
//! ## Distinctive Features
//!
//! A Phoneme can be decomposed into a collection of abstract features that
//! describe that sound. These features are based on perceptible qualities that
//! exist in a phoneme's sound wave based on the physical/mechanical means by
//! which a speaker produces sound. The \[nasal\] feature describes sounds
//! where air passes through the nasal tract. The \[+/-voice\] feature
//! describes sounds that are either voiced \[+voice\] or unvoiced \[-voice\].
//!
//! Feature systems are useful for a couple of reasons. First,
//! they take an otherwise arbitrary collection (the set of phonemes in a
//! language) and construct each element of the set from reusable, composable
//! elements. Phonemes can be selected and discussed as subsets then, based on
//! the underlying features they share.
//! Additionally, the behavior of phonemes and the ways they are transformed
//! in a system of language appear to be consistently explained by these
//! feature divisions. Transformations don't occur on arbitrary
//! subsets of phonemes, but instead on the natural classes of sounds formed
//! by selecting features (ex: all nasal sounds, all high vowels, etc).
//!
//! There are a variety of systems for describing distinctive phonological
//! features. The one represented here uses both [`Binary`] and [`Unary`]
//! features to describe how a language perceives and treats sounds. It dabbles
//! with autosegmental features, and it represents certain features as being
//! structurally dependent on other features (feature geometry).
//!
//! ## Autosegmental Features
//!
//! Some features appear to behave autonomously of their associated segments,
//! exhibiting behaviors where a feature does not associate in a one-to-one
//! manner with a segment. These sorts of autosegmental phenomena are
//! represented in a couple of ways in this module.
//!
//! [`Disegments`] are used to construct phonemes with a two-to-one
//! feature-segment relationship (diphthongs, affricates). This relationship is
//! constructed here not as a feature-segment relationship, however; instead it
//! is a segment-phoneme relationship. This is an opinionated choice, and seeks
//! to better align with the way that diphthongs and affricates appear as a
//! multiple segments behaving as a single segment.
//!
//! Autosegmental features are all marked as optional within the structure of a
//! segment. This can be used not only to describe phonemes where a feature is
//! absent, but also to construct under-specified sounds that take features
//! from nearby sounds in language production.
//!
//! ## Feature Geometry
//!
//! Features are organized into a tree-like structure in order to represent
//! the material dependencies between some features. The rounding of the lips
//! (\[round\]) is dependent on the presence of a \[labial\] feature, which is
//! to say that it is dependent upon the use of lips. Transformations will
//! often target a parent node in the feature geometry, affecting not only that
//! feature, but all child nodes underneath that feature.
//!
//! The feature geometry used to represent segments in this module is
//! depicted in the diagram below:
//!
//!<pre>
//!  [round]  [+/-anterior][+/-distib]  [+/-high][+/-low][+/-back]  [+/-ATR]
//!     |                |    |                  \    |   /             |
//!  [labial]           [coronal]                 [dorsal]        [pharyngeal]
//!      \__________________|_________________________|________________/
//!                                      |
//!                                    place
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
//!
//! [`Unary`]: enum.UnaryFeature.html
//! [`Binary`]: enum.BinaryFeature.html
//! [`Disegments`]: ../phoneme/enum.Phoneme.html

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
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub enum UnaryFeature {
    ///The feature is notably present on the segment
    Marked,
}

///A Segment is a structured collection of phonological features used to
///describe a Phoneme.
///
///All segments have a collection of Root Features that are bound to the
///segment. Autosegmental Features are more fluid, and only a subset of all
///autosegmental features are specified for any segment. These features behave
///differently from root features when a segment undergoes a phonological
///transformation.
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct LabialFeature {
    ///rounding of the lips during sound production: round vowels (+).
    pub round: Option<UnaryFeature>,
}

///Features determined by behavior involving the front of the tongue.
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
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
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct DorsalFeature {
    ///high tongue position: high vowels (+); mid and low vowels (-).
    pub high: Option<BinaryFeature>,
    ///low tongue position: low vowels (+); mid and high vowels (-).
    pub low: Option<BinaryFeature>,
    ///tongue is not front: back and central vowels (+); front vowels (-).
    pub back: Option<BinaryFeature>,
}

///Features determined by behavior at the root of the tongue.
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct PharyngealFeature {
    ///tongue root is forward. doubles as [+/-tense]. 'i', 'e', 'u', 'o' (+).
    ///ATR should be undefined for low vowels.
    pub advanced_tongue_root: Option<BinaryFeature>,
}

///Features determined by the behavior of the vocal folds.
#[derive(PartialEq, Eq, Debug, Default, Hash, Clone)]
pub struct LaryngealFeatures {
    ///open vocal folds: aspirated segments.
    pub spread_glottis: Option<UnaryFeature>,
    ///constricted vocal folds: ejectives, glottal stops.
    pub constricted_glottis: Option<UnaryFeature>,
    ///vibrating vocal folds: 'b', 'd', 'ɡ' (+); 'p', 't', 'k' (-)
    pub voice: Option<BinaryFeature>,
}
