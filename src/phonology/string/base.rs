use std::ops::Range;

use crate::phonology::{
    segment::SegmentFeatures, syllable::SyllableFeatures, tree::UniformDepth3Tree,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub(crate) tree: UniformDepth3Tree<(), SyllableFeatures, SegmentFeatures>,
}

impl PhonoString {
    pub fn new(tree: UniformDepth3Tree<(), SyllableFeatures, SegmentFeatures>) -> Self {
        Self { tree }
    }

    pub fn replace_range(
        mut self,
        range: Range<usize>,
        replace_with: PhonoString,
    ) -> Result<Self, String> {
        self.tree = self.tree.replace_range(range, replace_with.tree)?;

        Ok(self)
    }
}
