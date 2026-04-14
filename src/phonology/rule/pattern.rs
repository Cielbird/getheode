use std::ops::Range;

use crate::phonology::{
    rule::{SegmentInfo, SyllableInfo, TaggedPhonoString},
    string::PhonoString,
    tree::Depth3Tree,
};

pub struct PhonoStringPattern {
    pub(crate) tree: TaggedPhonoString,

    pub(crate) left_bound: PatternBorder,
    pub(crate) right_bound: PatternBorder,
}

impl PhonoStringPattern {
    pub fn new(tree: Depth3Tree<(), SyllableInfo, SegmentInfo>) -> Self {
        Self {
            tree: TaggedPhonoString::new(tree),
            left_bound: PatternBorder::Any,
            right_bound: PatternBorder::Any,
        }
    }
}

pub enum PatternBorder {
    Word,
    StrictSyllable, // may only be a syllable boundary, not more or less
    StrictSegment,  // may only be a segment boundary, not more
    SyllableOrWord, // may only be a syllable or word boundary
    Any,            // may only be a segment, syllable, or word boundary
}

impl PatternBorder {
    pub fn respects(&self, is_segment_on_border: bool, is_syllable_on_border: bool) -> bool {
        match self {
            PatternBorder::Word => is_segment_on_border && is_syllable_on_border,
            PatternBorder::SyllableOrWord => is_segment_on_border,
            PatternBorder::StrictSyllable => is_segment_on_border && !is_syllable_on_border,
            PatternBorder::StrictSegment => !is_segment_on_border,
            PatternBorder::Any => true,
        }
    }
}

pub struct PatternMatch {
    pub range: Range<usize>,
    pub replace_with: PhonoString,
}
