use crate::phonology::{segment::PhonoSegment, syllable::SyllableFeatures};

pub struct PhonoPattern {
    pub(crate) elems: Vec<PhonoPatternElement>,
}

impl PhonoPattern {
    pub fn new<E>(elems: E) -> Self
    where
        E: IntoIterator<Item: Into<PhonoPatternElement>>,
    {
        Self {
            elems: elems.into_iter().map(|e| e.into()).collect(),
        }
    }
}

// A pattern for matching phonological strings
pub enum PhonoPatternElement {
    WordBoundary,
    SyllableBoundary,
    Segment(PhonoPatternSegment),
}

// A segment element of a pattern for phonological strings
pub struct PhonoPatternSegment {
    pub(crate) syllable_features: SyllableFeatures,
    pub(crate) segment: PhonoSegment, // segment is only composed of it's features
}

/// Convert (syllable_features, segment_features) to pattern element
impl<SYL, SEG> From<(SYL, SEG)> for PhonoPatternElement
where
    SYL: Into<SyllableFeatures>,
    SEG: Into<PhonoSegment>,
{
    fn from(tup: (SYL, SEG)) -> Self {
        PhonoPatternElement::Segment(tup.into())
    }
}

impl<SYL, SEG> From<(SYL, SEG)> for PhonoPatternSegment
where
    SYL: Into<SyllableFeatures>,
    SEG: Into<PhonoSegment>,
{
    fn from(tup: (SYL, SEG)) -> Self {
        let (syllable_features, segment) = tup;
        let syllable_features = syllable_features.into();
        let segment = segment.into();
        PhonoPatternSegment {
            syllable_features,
            segment,
        }
    }
}
