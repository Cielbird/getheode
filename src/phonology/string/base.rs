use crate::phonology::{
    pattern::{
        PhonoPattern,
        PhonoPatternElement::{Segment, SyllableBoundary, WordBoundary},
    },
    syllable::PhonoSyllable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct PhonoString {
    pub(crate) syllables: Vec<PhonoSyllable>,
}

impl PhonoString {
    /// construct a string from syllables
    pub fn new(syllables: impl IntoIterator<Item = impl Into<PhonoSyllable>>) -> Self {
        PhonoString {
            syllables: syllables.into_iter().map(|s| s.into()).collect(),
        }
    }

    pub fn replace(self, _start: usize, _end: usize, replacement: PhonoPattern) -> Self {
        for e in replacement.elems {
            match e {
                WordBoundary => todo!(),
                SyllableBoundary => todo!(),
                Segment(seg) => {
                    let _seg_feats = seg.segment;
                    let _syl_feats = seg.syllable_features;
                }
            }
        }
        // TODO
        self
    }
}
