use crate::phonology::rule::{SegmentInfo, SyllableInfo};

/// a boundary or a feature set for a segment
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    Features(SyllableInfo, SegmentInfo),
    WordBoundary,
    SyllableBoundary,
    Null,
}

pub struct ElemSequence {
    pub elems: Vec<Element>
}
