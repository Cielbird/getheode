use std::ops::Add;
use crate::phonology::syllable::{SYL_FEATURE_COUNT};
use crate::phonology::feature::FeatureState::{self, *};


/// set of features a syllable can have
#[derive(Debug, Clone, PartialEq)]
pub struct SyllableFeatures {
    pub features: [FeatureState; SYL_FEATURE_COUNT as usize],
}

impl SyllableFeatures {
    pub const fn new(features: [FeatureState; SYL_FEATURE_COUNT as usize]) -> Self {
        Self { features }
    }

    /// returns true if this segment matches `pattern`'s defined features.  
    /// to return true:
    /// - if a feature is defined in `pattern`, it must be defined in this segment
    /// - if a feature is `POS` or `NEG` in `pattern`, it must be identical in this segment
    /// - if a features is `NA` in `pattern`, it can be `POS`, `NEG`, or `NA` in this segment
    ///
    /// otherwise, returns false.
    pub fn matches(&self, pattern: &SyllableFeatures) -> bool {
        for i in 0..(SYL_FEATURE_COUNT as usize) {
            if pattern.features[i] == UNDEF || pattern.features[i] == self.features[i] {
                continue;
            } else {
                return false;
            }
        }

        true
    }
}

impl From<[FeatureState; SYL_FEATURE_COUNT as usize]> for SyllableFeatures {
    fn from(features: [FeatureState; SYL_FEATURE_COUNT as usize]) -> Self {
        Self { features }
    }
}

impl Add<SyllableFeatures> for SyllableFeatures {
    type Output = Self;

    /// adds the features of the rhs segment to the lhs segment.
    /// if the rhs segment is complete (completely defined), the result is the rhs.
    /// if the feature is defined in the rhs, it will be overwritten in result,
    /// otherwise, the lsh's feature value will be used.
    fn add(self, s2: Self) -> Self {
        let mut result = self.clone();
        for i in 0..(SYL_FEATURE_COUNT as usize) {
            if s2.features[i] != UNDEF {
                result.features[i] = s2.features[i];
            }
        }
        result
    }
}
