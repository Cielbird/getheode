use std::ops::Range;

use crate::phonology::{
    rule::{SegmentInfo, SyllableInfo, TaggedPhonoString},
    string::PhonoString,
    tree::Depth3Tree,
};

#[derive(Debug)]
pub struct PhonoStringPattern {
    pub tree: TaggedPhonoString,

    pub left_bound: PatternBorder,
    pub right_bound: PatternBorder,
}

impl PhonoStringPattern {
    pub fn new(
        tree: Depth3Tree<(), SyllableInfo, SegmentInfo>,
        left_bound: PatternBorder,
        right_bound: PatternBorder,
    ) -> Self {
        Self {
            tree: TaggedPhonoString::new(tree),
            left_bound,
            right_bound,
        }
    }
}

#[derive(Debug)]
pub enum PatternBorder {
    Word,
    StrictSyllable, // may only be a syllable boundary, not more or less
    StrictSegment,  // may only be a segment boundary, not more
    SyllableOrWord, // may only be a syllable or word boundary
    Any,            // may only be a segment, syllable, or word boundary
}

impl PatternBorder {
    pub fn respects(&self, on_syllable_border: bool, on_word_border: bool) -> bool {
        match self {
            PatternBorder::Word => on_syllable_border && on_word_border,
            PatternBorder::SyllableOrWord => on_syllable_border,
            PatternBorder::StrictSyllable => on_syllable_border && !on_word_border,
            PatternBorder::StrictSegment => !on_syllable_border,
            PatternBorder::Any => true,
        }
    }
}

pub struct PatternMatch {
    pub range: Range<usize>,
    pub replace_with: PhonoString,
}
