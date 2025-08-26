use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::{
    error::*,
    segment::{
        DIACRITICS, FEATURE_NAMES, FeatureState, IPA_BASES, NATURAL_CLASSES, Segment,
        feature_from_string,
    },
};

/// Defines the string format for ipa segments
pub trait FormatIpa {
    /// Parse an ipa character segment
    fn parse_ipa(input: &str) -> Result<Self>
    where
        Self: Sized;

    /// Format as an ipa character string
    fn format_ipa(&self) -> String;
}

/// Defines the formatting of a phonological feature set
pub trait FormatFeatureSet {
    /// Parse an ipa character segment
    fn parse_feature_set(input: &str) -> Result<Self>
    where
        Self: Sized;

    /// Format as an ipa character string
    fn format_feature_set(&self) -> String;
}

/// Defines the formatting of an incomplete segment as a phonological class string
pub trait FormatPhonologicalClass {
    /// Parse an ipa character segment
    fn parse_class(input: &str) -> Result<Self>
    where
        Self: Sized;

    /// Format as an ipa character string
    fn format_class(&self) -> String;
}

/// Defines how a segment can be parsed and formatted.
impl Segment {
    /// return a segment from either an ipa character, a feature set, or a phonological class.
    /// - input string is trimmed of whitespace
    pub fn parse_segment(string: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let string = string.trim();
        if let Ok(seg) = Self::parse_ipa(string) {
            return Ok(seg);
        }
        if let Ok(seg) = Self::parse_class(string) {
            return Ok(seg);
        }
        if let Ok(seg) = Self::parse_feature_set(string) {
            return Ok(seg);
        }
        Err(Error::SegmentParsingError(string.to_string()))
    }
    
    pub fn format_segment(&self) -> String {
        // see if there is a matching ipa symbol
        for (sym, seg) in IPA_BASES {
            if seg == self {
                return sym.to_string();
            }
            // WARNING this tries all possible ipa symbols with all possible diacritics.
            // not only is it limited to only one diacritic, but it is extremely slow, in theory.
            // for now, there are only a handfull of diacritics. the algorithm to do this well and
            // fast is too much for me to think of right now; a fun puzzle for later.
            // TODO tackle this when performance becomes important, or when i need multiple
            // diacritics
            // TODO this can be done recursively
            for (d, d_seg) in DIACRITICS {
                // TODO figure out if cloning these is really what i'm supposed to do
                if (seg.clone() + d_seg.clone()) == *self {
                    let mut s = sym.to_string();
                    s.push(*d);
                    return s.to_string();
                }
            }
        }

        // see if there is a matching class
        for (sym, seg) in NATURAL_CLASSES {
            if seg == self {
                return sym.to_string();
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

        (result + "]").to_string()
    }
}

impl FormatIpa for Segment {
    /// construct a segement from an IPA symbol
    /// see https://www.unicode.org/reports/tr15/#Canon_Compat_Equivalence
    fn parse_ipa(input: &str) -> Result<Self> {
        for (symbol, seg) in IPA_BASES {
            // normalize the
            let mut input_norm = input.nfd();
            let symbol_norm = symbol.nfd();

            // do the first utf8 code points match the first of our symbol?
            let mut matches = true;
            for symbol_char in symbol_norm {
                if input_norm.next() != Some(symbol_char) {
                    matches = false;
                    break;
                }
            }
            if !matches {
                // try next symbol
                continue;
            }

            // collect remaining (unchecked) characters
            let remaining = input_norm.collect::<String>();
            // if there are no diacritics to add it will do nothing
            match parse_remaining(&remaining, seg.clone()) {
                Ok(segment) => {
                    return Ok(segment);
                }
                Err(_e) => continue,
            }
        }
        let msg = format!("The symbol {} could not be parsed", input);
        Err(Error::IPASymbolParsingError(msg))
    }

    fn format_ipa(&self) -> String {
        todo!()
    }
}

/// recursive function to add the ipa diacritics in a string to a segment
fn parse_remaining(remaining_chars: &str, cur_segment: Segment) -> Result<Segment> {
    if remaining_chars.is_empty() {
        return Ok(cur_segment);
    }
    if remaining_chars.starts_with('(') && remaining_chars.ends_with(')') {
        let remaining_chars = &remaining_chars[1..(remaining_chars.len() - 1)];

        return Ok(cur_segment + parse_features(remaining_chars)?);
    }
    for (symbol, diac) in DIACRITICS {
        // do the first utf8 code points match the first of our symbol?
        // normalize the
        let mut input_norm = remaining_chars.nfd();
        let symbol_norm = symbol.nfd();

        // do the first utf8 code points match the first of our symbol?
        let mut matches = true;
        for symbol_char in symbol_norm {
            if input_norm.next() != Some(symbol_char) {
                matches = false;
                break;
            }
        }
        if !matches {
            // try next symbol
            continue;
        }

        // collect remaining (unchecked) characters
        let remaining = &input_norm.collect::<String>();
        let new_seg = cur_segment.clone() + diac.clone();
        match parse_remaining(remaining, new_seg) {
            Ok(segment) => {
                return Ok(segment);
            }
            Err(_e) => continue,
        }
    }
    let msg = format!("The symbol {} could not be parsed", remaining_chars);
    Err(Error::IPASymbolParsingError(msg))
}

impl Segment {
    /// construct a segement from an IPA symbol
    pub fn parse_class(class_symbol: &str) -> Result<Self> {
        for (sym, seg) in NATURAL_CLASSES {
            if *sym == class_symbol {
                return Ok(seg.clone());
            }
        }
        Err(Error::IPASymbolParsingError(class_symbol.to_string()))
    }

    pub fn format_class(&self) -> String {
        todo!()
    }
}

impl Segment {
    /// construct a segement from a list of features in brackets ex. [+voi-delrel]
    pub fn parse_feature_set(s: &str) -> Result<Self> {
        let s = s.trim();
        if !(s.starts_with('[') && s.ends_with(']')) {
            return Err(Error::SegmentParsingError(s.to_string()));
        }
        let inner = &s[1..(s.len() - 1)];
        parse_features(inner)
    }

    pub fn format_feature_set(&self) -> String {
        todo!()
    }
}

/// Parse a string of feature states, for example "+voi -spgl"
fn parse_features(mut features_str: &str) -> Result<Segment> {
    let mut seg = Segment::new_undef();
    let re = Regex::new(r"^\s*([+-])\s*([a-z]+)").unwrap();
    while !features_str.is_empty() {
        let sign: char;
        let name_match;
        let name: &str;
        match re.captures(features_str) {
            Some(capts) => {
                sign = capts[1].chars().nth(0).unwrap();
                name_match = capts.get(2).unwrap();
                name = &features_str[name_match.range()];
            }
            None => {
                return Err(Error::SegmentParsingError(features_str.to_string()));
            }
        }
        let feature = match feature_from_string(name) {
            Ok(f) => f,
            Err(e) => {
                return Err(e);
            }
        };
        // set feature
        if sign == '+' {
            seg.features[feature as usize] = FeatureState::POS;
        } else if sign == '-' {
            seg.features[feature as usize] = FeatureState::NEG;
        }

        features_str = features_str[name_match.end()..features_str.len()].trim();
    }
    Ok(seg)
}
