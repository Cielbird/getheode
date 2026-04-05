#![allow(unused)] // TODO remove once this mod is used

use crate::phonology::{
    segment::SegmentFeatures, syllable::SyllableFeatures, tree::UniformDepth3Tree,
};

/// A pattern to match in phonological strings
pub struct PhonoPattern {
    pub(crate) options: Vec<PhonoPatternOpt>,
}

pub struct PhonoPatternOpt {
    // use a tree to represent the string, like phonological strings
    pub(crate) tree: UniformDepth3Tree<(), SyllableInfo, SegmentInfo>,
}

pub struct SyllableInfo {
    pub(crate) id: u32,
    pub(crate) features: SyllableFeatures,
}

impl SyllableInfo {
    pub fn new(id: u32, features: SyllableFeatures) -> Self {
        Self { id, features }
    }
}

pub struct SegmentInfo {
    pub(crate) id: u32,
    pub(crate) features: SegmentFeatures,
}

impl SegmentInfo {
    pub fn new(id: u32, features: SegmentFeatures) -> Self {
        Self { id, features }
    }
}

impl PhonoPattern {
    pub(crate) fn new(options: Vec<PhonoPatternOpt>) -> Self {
        Self { options }
    }
}
