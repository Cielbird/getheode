use core::fmt;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

use crate::phonology::{
    feature::{
        Feature,
        FeatureState::{self, NEG, POS, UNDEF},
    },
    segment::{SEG_FEATURE_COUNT, format_segment},
};

/// represents a set of phonological features
///
/// can represent either a complete phonological segment (if all features are defined)
/// or a set of features that can be used to match or modify other segments
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub struct SegmentFeatures {
    pub(crate) features: [FeatureState; SEG_FEATURE_COUNT as usize],
}

impl SegmentFeatures {
    /// construct a segement from an array of features
    pub const fn from_features(features: [FeatureState; SEG_FEATURE_COUNT as usize]) -> Self {
        SegmentFeatures { features }
    }

    /// construct a segement with all features undefied
    pub const fn new_undef() -> Self {
        SegmentFeatures {
            features: [FeatureState::UNDEF; SEG_FEATURE_COUNT as usize],
        }
    }

    /// returns true if the segment is complete, ie, completely defined for all features.
    /// if false, this segment is a just set of features,
    /// usually used for matching or modifying other segments
    pub fn is_complete(&self) -> bool {
        for i in 0..(SEG_FEATURE_COUNT as usize) {
            if self.features[i] == UNDEF {
                return false;
            }
        }
        true
    }

    /// returns true if this segment matches `pattern`'s defined features.  
    /// to return true:
    /// - if a feature is defined in `pattern`, it must be defined in this segment
    /// - if a feature is `POS` or `NEG` in `pattern`, it must be identical in this segment
    /// - if a features is `NA` in `pattern`, it can be `POS`, `NEG`, or `NA` in this segment
    ///
    /// otherwise, returns false.
    pub fn matches(&self, pattern: &SegmentFeatures) -> bool {
        for i in 0..(SEG_FEATURE_COUNT as usize) {
            if pattern.features[i] == UNDEF || pattern.features[i] == self.features[i] {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    /// returns the number of features that would have to change
    /// to make the lhs segment equal to the rhs one
    fn _dist(&self, other: &SegmentFeatures) -> u8 {
        let mut dist = 0;
        for i in 0..(SEG_FEATURE_COUNT as usize) {
            if self.features[i] != other.features[i] {
                dist += 1;
            }
        }

        dist
    }
}

impl Add<SegmentFeatures> for SegmentFeatures {
    type Output = Self;

    /// adds the features of the rhs segment to the lhs segment.
    /// if the rhs segment is complete (completely defined), the result is the rhs.
    /// if the feature is defined in the rhs, it will be overwritten in result,
    /// otherwise, the lsh's feature value will be used.
    fn add(self, s2: Self) -> Self {
        let mut result = self.clone();
        for i in 0..(SEG_FEATURE_COUNT as usize) {
            if s2.features[i] != UNDEF {
                result.features[i] = s2.features[i];
            }
        }
        result
    }
}

impl Add<Feature> for SegmentFeatures {
    type Output = Self;

    /// adds the feature to the segment; sets the feature to `POS`
    fn add(self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = POS;

        result
    }
}

impl Sub<Feature> for SegmentFeatures {
    type Output = Self;

    // removes the feature from the segment: sets the feature to `NEG`
    fn sub(self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = NEG;

        result
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for SegmentFeatures {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_segment(self))
    }
}
