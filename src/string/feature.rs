

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum FeatureState {
    #[default]
    UNDEF, // undefined for this segment.
    POS, // (+) present in the segment
    NEG, // (-) not present in the segment
    NA,  // not applicable, could be either positive or negative for this segment, we don't care
}

// useful for both syllables and segments
pub type Feature = u8;

