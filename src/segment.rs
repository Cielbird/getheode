use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::diacritics::DIACRITICS;
use crate::error::{Error, Result};
use crate::feature::FeatureState::*;
use crate::feature::{feature_from_string, Feature, FeatureState, FEATURE_COUNT, FEATURE_NAMES};
use crate::ipa_segments::IPA_BASES;
use crate::natural_classes::NATURAL_CLASSES;
use crate::segment_string::SegmentString;
use core::fmt;
use std::{
    fmt::Display,
    ops::{Add, Sub},
};

/// represents a set of phonological features
/// 
/// can represent either a complete phonological segment (if all features are defined)
/// or a set of features that can be used to match or modify other segments
#[derive(Debug, Clone, Default, Eq, Hash, PartialEq)]
pub struct Segment {
    features: [FeatureState; FEATURE_COUNT as usize],
}

impl Segment {
    /// construct a segement from an array of features
    pub const fn from_features(features: [FeatureState; FEATURE_COUNT as usize]) -> Self {
        Segment { features: features }
    }

    /// construct a segement with all features undefied
    pub const fn new_undef() -> Self {
        return Segment {
            features: [FeatureState::UNDEF; FEATURE_COUNT as usize],
        };
    }

    /// return a segment from either an ipa character
    /// or a feature list in brackets ex. [+voi-delrel]
    /// - input string is trimmed of whitespace
    pub fn from_string(string: &str) -> Result<Self> {
        let string = string.trim();
        if let Ok(seg) = Self::from_ipa(string) {
            return Ok(seg);
        }
        if let Ok(seg) = Self::from_class(string) {
            return Ok(seg);
        }
        if let Ok(seg) = Self::from_features_string(string) {
            return Ok(seg);
        }
        Err(Error::SegmentParsingError(string.to_string()))
    }

    /// construct a segement from an IPA symbol
    /// see https://www.unicode.org/reports/tr15/#Canon_Compat_Equivalence
    fn from_ipa(ipa_symbol: &str) -> Result<Self> {
        for (symbol, seg) in IPA_BASES {
            // do the first utf8 code points match the first of our symbol?
            let mut ipa_sym = ipa_symbol.nfd();
            let mut sym = symbol.nfd();
            let matches = sym
                .all(|prefix_item| 
                    ipa_sym.next()
                            .map_or(false, |item| item == prefix_item));
            if !matches {
                // try next symbol
                continue;
            }
            // collect remaining (unchecked) characters
            let remaining = &ipa_sym.collect::<String>();
            match Self::add_diacritics(remaining, seg) {
                Ok(segment) => {
                    return Ok(segment);
                },
                Err(_e) => continue
            }
        }
        let msg = format!("The symbol {} could not be parsed", ipa_symbol);
        return Err(Error::IPASymbolParsingError(msg))
    }

    /// recursive function to add the diacritics in a string to a segment
    fn add_diacritics(remaining_chars: &str, cur_segment: &Segment) -> Result<Self>{
        if remaining_chars.len() == 0 {
            return Ok(cur_segment.clone());
        }
        for (symbol, diac) in DIACRITICS {
            // do the first utf8 code points match the first of our symbol?
            let mut ipa_sym = remaining_chars.nfd();
            let mut sym = symbol.nfd();
            let matches = sym
                .all(|prefix_item| 
                    ipa_sym.next()
                            .map_or(false, |item| item == prefix_item));
            if !matches {
                // try next symbol
                continue;
            }
            // collect remaining (unchecked) characters
            let remaining = &ipa_sym.collect::<String>();
            let new_seg = cur_segment.clone() + diac.clone();
            match Self::add_diacritics(remaining, &new_seg) {
                Ok(segment) => {
                    return Ok(segment);
                },
                Err(_e) => continue
            }
        }
        let msg = format!("The symbol {} could not be parsed", remaining_chars);
        return Err(Error::IPASymbolParsingError(msg))
    }

    /// construct a segement from an IPA symbol
    fn from_class(class_symbol: &str) -> Result<Self> {
        for (sym, seg) in NATURAL_CLASSES{
            if *sym == class_symbol {
                return Ok(seg.clone());
            }
        }
        return Err(Error::IPASymbolParsingError(
            class_symbol.to_string(),
        ));
    }

    /// construct a segement from a list of features in brackets ex. [+voi-delrel]
    fn from_features_string(s: &str) -> Result<Self> {
        let s = s.trim();
        if !(s.starts_with('[') && s.ends_with(']')) {
            return Err(Error::SegmentParsingError(s.to_string()));
        }
        let mut inner = &s[1..(s.len() - 1)];
        let mut seg = Self::new_undef();
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
                None => {
                    return Err(Error::SegmentParsingError(inner.to_string()));
                }
            }
            let feature;
            match feature_from_string(name) {
                Ok(f) => feature = f,
                Err(e) => {
                    return Err(e);
                }
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

    /// return a SegmentString with a single segment: a clone of this segment.
    pub fn to_seg_string(&self) -> SegmentString {
        SegmentString::from_segments(vec![self.clone()])
    }

    /// returns true if the segment is complete, ie, completely defined for all features.
    /// if false, this segment is a just set of features,
    /// usually used for matching or modifying other segments
    pub fn is_complete(&self) -> bool {
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

    /// returns the number of features that would have to change
    /// to make the lhs segment equal to the rhs one
    fn dist(&self, other: &Segment) -> u8 {
        let mut dist = 0;
        for i in 0..(FEATURE_COUNT as usize) {
            if self.features[i] != other.features[i] {
                dist += 1;
            }
        }
        return dist;
    }
}

impl Add<Segment> for Segment {
    type Output = Self;

    /// adds the features of the rhs segment to the lhs segment.
    /// if the rhs segment is complete (completely defined), the result is the rhs.
    /// if the feature is defined in the rhs, it will be overwritten in result,
    /// otherwise, the lsh's feature value will be used.
    fn add(self, s2: Self) -> Self {
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

    /// adds the feature to the segment; sets the feature to `POS`
    fn add(self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = POS;
        return result;
    }
}

impl Sub<Feature> for Segment {
    type Output = Self;

    // removes the feature from the segment: sets the feature to `NEG`
    fn sub(self, feature: Feature) -> Self {
        let mut result = self.clone();
        result.features[feature as usize] = NEG;
        return result;
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // see if there is a matching ipa symbol
        for (sym, seg) in IPA_BASES {
            if seg == self {
                return write!(f, "{}", sym);
            }
            // WARNING this tries all possible ipa symbols with all possible diacritics.
            // not only is it limited to only one diacritic, but it is extremely slow, in theory.
            // for now, there are only a handfull of diacritics. the algorithm to do this well and
            // fast is too much for me to think of right now; a fun puzzle for later.
            // TODO tackle this when performance becomes pertinent, or when i need multiple
            // diacritics
            // TODO this can be done recursively
            for (d, d_seg) in DIACRITICS {
                // TODO figure out if cloning these is really what i'm supposed to do
                if (seg.clone() + d_seg.clone()) == *self {
                    let mut s = sym.to_string();
                    s.push(*d);
                    return write!(f, "{}", s);
                }
            }
        }

        // see if there is a matching class
        for (sym, seg) in NATURAL_CLASSES {
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
