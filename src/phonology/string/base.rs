use crate::phonology::{
    segment::SegmentFeatures, syllable::SyllableFeatures, tree::UniformDepth3Tree,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub(crate) tree: UniformDepth3Tree<(), SyllableFeatures, SegmentFeatures>,
}

impl PhonoString {
    pub(crate) fn new(x: Vec<Vec<(SyllableFeatures, Vec<SegmentFeatures>)>>) -> Self {
        let v2 = x.into_iter().map(|v| ((), v)).collect();
        let tree = UniformDepth3Tree::new(v2);
        Self { tree }
    }
}
