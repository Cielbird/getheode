use crate::phonology::{
    segment::SegmentFeatures, syllable::SyllableFeatures, tree::UniformDepth3Tree,
};

/// A pattern to match in phonological strings
pub struct PhonoPattern {
    pub(crate) options: Vec<PhonoPatternOpt>,
}

pub struct PhonoPatternOpt {
    // use a tree to represent the string, like phonological strings
    tree: UniformDepth3Tree<(), SyllableInfo, SegmentInfo>,
}

pub struct SyllableInfo {
    id: CaptureId,
    features: SyllableFeatures,
}

pub struct SegmentInfo {
    id: CaptureId,
    segment: SegmentFeatures,
}

pub struct CaptureId(u32);

impl PhonoPattern {}
