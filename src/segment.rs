use regex::Regex;

/// represents a set of phonological features
/// can represent either a complete phonological segment (if all features are defined)
/// or a set of features that can be used to match or modify other segments

use crate::{feature::{feature_from_string, Feature, FeatureState, FEATURE_COUNT, FEATURE_NAMES}, ipa_segments::IPA_BASES, segment_string::SegmentString};
use crate::feature::FeatureState::*;
use core::fmt;
use std::{fmt::Display, ops::{Add, Sub}};
use crate::errors::GetheodeError;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Segment {
    features: [FeatureState; FEATURE_COUNT as usize]
}

impl Segment {
    /// return a segment from either an ipa character 
    /// or a feature list in brackets ex. [+voi-delrel]
    pub fn from_string(string: &str) -> Result<Self, GetheodeError> {
        if let Ok(seg) = Self::from_ipa(string) {
            return Ok(seg);
        }
        if let Ok(seg) = Self::from_features_string(string) {
            return Ok(seg);
        }
        Err(GetheodeError::SegmentParsingError(string.to_string()))
    }

    /// construct a segement from an IPA symbol
    pub fn from_ipa(ipa_symbol: &str) -> Result<Self, GetheodeError> {
        for (sym, seg) in IPA_BASES {
            if *sym == ipa_symbol {
                return Ok(seg.clone());
            }
        }
        return Err(GetheodeError::IPASymbolParsingError(ipa_symbol.to_string()));
    }

    /// construct a segement from a list of features in brackets ex. [+voi-delrel]
    pub fn from_features_string(s: &str) -> Result<Self, GetheodeError> {
        let s = s.trim();
        if !(s.starts_with('[') && s.ends_with(']')) {
            return Err(GetheodeError::SegmentParsingError(s.to_string()));
        }
        let mut inner = &s[1..(s.len() - 1)];
        let mut seg = Self{features: [FeatureState::UNDEF; FEATURE_COUNT as usize]};
        let re = Regex::new(r"^\s*([+-])\s*([a-z]+)").unwrap();
        while inner != "" {
            let sign: char;
            let name_match;
            let name: &str;
            match re.captures(inner) {
                Some(capts) => {
                    sign = capts[1].chars().nth(0).unwrap();
                    name_match = capts.get(2).unwrap();
                    name = &inner[name_match.range()];
                }
                None => { return Err(GetheodeError::SegmentParsingError(inner.to_string())); }
            }
            let feature;
            match feature_from_string(name) {
                Ok(f) => feature = f,
                Err(e) => { return Err(e); },
            }
            // set feature
            if sign == '+' {
                seg.features[feature as usize] = FeatureState::POS;
            } else if sign == '-' {
                seg.features[feature as usize] = FeatureState::NEG;
            }

            inner = &inner[name_match.end()..inner.len()].trim();
        }
        return Ok(seg);
    }

    /// construct a segement from an array of features
    pub const fn from_features(features: [FeatureState; FEATURE_COUNT as usize]) -> Self {
        Segment { features: features }
    }

    /// return a SegmentString with a single segment: a clone of this segment.
    pub fn to_seg_string(self) -> SegmentString {
        SegmentString::from_segments(vec![self.clone()])
    }

    /// returns true if the segment is complete, ie, completely defined for all features.
    /// if false, this segment is a just set of features, 
    /// usually used for matching or modifying other segments 
    pub fn is_complete(self) -> bool {
        for i in 0..(FEATURE_COUNT as usize) {
            if self.features[i] == UNDEF {
                return false;
            }
        }
        return true;
    }

    /// returns true if this segment matches `other`'s defined features.  
    /// to return true: 
    /// - if a feature is defined in `other`, it must be defined in this segment
    /// - if a feature is `POS` or `NEG` in `other`, it must be identical in this segment
    /// - if a features is `NA` in `other`, it can be `POS`, `NEG`, or `NA` in this segment
    /// 
    /// otherwise, returns false.
    pub fn matches(&self, other: &Segment) -> bool {
        for i in 0..(FEATURE_COUNT as usize) {
            if other.features[i] == UNDEF {
                continue;
            } else if other.features[i] == self.features[i] {
                continue;
            } else {
                return false;
            }
        }
        return true;
    }
}

impl Add<Segment> for Segment {
    type Output = Self;

    /// adds the features of the rhs segment to the lhs segment. 
    /// if the rhs segment is complete (completely defined), the result is the rhs.
    /// if the feature is defined in the rhs, it will be overwritten in result,
    /// otherwise, the lsh's feature value will be used.
    fn add (self, s2: Self) -> Self {
        let mut result = self.clone();
        for i in 0..(FEATURE_COUNT as usize) {
            if s2.features[i] != UNDEF {
                result.features[i] = s2.features[i].clone();
            }
        }
        return result;
    }
}

impl Add<Feature> for Segment {
    type Output = Self;
    
    /// adds the features to the segment 
    fn add (self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = POS;
        return result;
    }
}

impl Sub<Feature> for Segment {
    type Output = Self;
    
    // removes the feature from the segment 
    fn sub (self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = NEG;
        return result;
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for Segment {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // see if there is a matching ipa char
        for (sym, seg) in IPA_BASES {
            if seg == self {
                return write!(f, "{}", sym);
            }
        }

        // otherwise spit out a list of the features
        let mut result: String = "[".to_string();
        for (i, feature) in FEATURE_NAMES.iter().enumerate() {
            if self.features[i] == FeatureState::NA {
                continue;
            } else if self.features[i] == FeatureState::POS {
                result = result + "+" + feature;
            } else if self.features[i] == FeatureState::NEG {
                result = result + "-" + feature;
            } 
        }
        return write!(f, "{}", result + "]");
    }
}
