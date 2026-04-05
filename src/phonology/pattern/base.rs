#![allow(unused)] // TODO remove once this mod is used

use crate::phonology::{
    pattern::PatternMatch, segment::SegmentFeatures, string::PhonoString, syllable::SyllableFeatures, tree::UniformDepth3Tree
};

pub type PatternTree = UniformDepth3Tree<(), SyllableInfo, SegmentInfo>;

/// A pattern to match in phonological strings
pub struct PhonoPattern {
    // use a tree to represent the string, like phonological strings
    pub(crate) match_tree: PatternTree,
    pub(crate) replace_tree: PatternTree,
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
    pub(crate) fn new(match_tree: PatternTree, replace_tree: PatternTree) -> Self {
        Self {
            match_tree,
            replace_tree,
        }
    }
    
    pub(crate) fn find(&self, hay: PhonoString) -> Vec<PatternMatch> {
        todo!()
    }
}
