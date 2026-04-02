use crate::string::feature::{self, FeatureState};

// The features that a syllable can have
pub const SYL_FEATURE_COUNT: u8 = 1;
// syllable features : [stress]

/// set of features a syllable can have
#[derive(Debug, Clone, PartialEq)]
pub struct SyllableFeatures {
    features : [FeatureState; SYL_FEATURE_COUNT as usize]
}

impl From<[FeatureState; SYL_FEATURE_COUNT as usize]> for SyllableFeatures {
    fn from(features: [FeatureState; SYL_FEATURE_COUNT as usize]) -> Self {
        Self {
            features
        }
    }
}
