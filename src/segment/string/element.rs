use crate::segment::Segment;

// an element of a phonological string : either a segment or a syllable/word boundary
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PhonologicalElement {
    SegmentElement(Segment),
    SyllableBoundary { stressed: bool },
    WordBoundary,
}

impl PhonologicalElement {
    pub fn matches(&self, other: &Self) -> bool {
        match self {
            PhonologicalElement::SegmentElement(segment) => {
                if let PhonologicalElement::SegmentElement(other) = other {
                    segment.matches(other)
                } else {
                    false
                }
            },
            PhonologicalElement::SyllableBoundary { stressed } => {
                if let PhonologicalElement::SyllableBoundary{stressed: other_stressed } = other {
                    *stressed == *other_stressed
                } else {
                    false
                }
            },
            PhonologicalElement::WordBoundary => {
                self == other 
            },
        }
    }
}
