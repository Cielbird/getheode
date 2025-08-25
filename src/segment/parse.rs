use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::{
    error::*,
    segment::{
        feature_from_string, FeatureState, Segment, DIACRITICS, FEATURE_NAMES, IPA_BASES, NATURAL_CLASSES
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
pub trait FormatSegment: FormatIpa + FormatFeatureSet + FormatPhonologicalClass {
    /// return a segment from either an ipa character, a feature set, or a phonological class.
    /// - input string is trimmed of whitespace
    fn parse_segment(string: &str) -> Result<Self>
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

    fn format_segment(&self) -> String {
        todo!()
    }
}

impl FormatSegment for Segment { 
    fn format_segment(&self) -> String {
        // see if there is a matching ipa symbol
        for (sym, seg) in IPA_BASES {
            if seg == self {
                return format!("{}", sym);
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
                    return format!("{}", s);
                }
            }
        }

        // see if there is a matching class
        for (sym, seg) in NATURAL_CLASSES {
            if seg == self {
                return format!("{}", sym);
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
        return format!("{}", result + "]");
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
            match parse_remaining_diacritics(&remaining, seg) {
                Ok(segment) => {
                    return Ok(segment);
                }
                Err(_e) => continue,
            }
        }
        let msg = format!("The symbol {} could not be parsed", input);
        return Err(Error::IPASymbolParsingError(msg));
    }

    fn format_ipa(&self) -> String {
        todo!()
    }
}

/// recursive function to add the ipa diacritics in a string to a segment
fn parse_remaining_diacritics(remaining_chars: &str, cur_segment: &Segment) -> Result<Segment> {
    if remaining_chars.len() == 0 {
        return Ok(cur_segment.clone());
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
        match parse_remaining_diacritics(remaining, &new_seg) {
            Ok(segment) => {
                return Ok(segment);
            }
            Err(_e) => continue,
        }
    }
    let msg = format!("The symbol {} could not be parsed", remaining_chars);
    return Err(Error::IPASymbolParsingError(msg));
}

impl FormatPhonologicalClass for Segment {
    /// construct a segement from an IPA symbol
    fn parse_class(class_symbol: &str) -> Result<Self> {
        for (sym, seg) in NATURAL_CLASSES {
            if *sym == class_symbol {
                return Ok(seg.clone());
            }
        }
        return Err(Error::IPASymbolParsingError(class_symbol.to_string()));
    }

    fn format_class(&self) -> String {
        todo!()
    }
}

impl FormatFeatureSet for Segment {
    /// construct a segement from a list of features in brackets ex. [+voi-delrel]
    fn parse_feature_set(s: &str) -> Result<Self> {
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

    fn format_feature_set(&self) -> String {
        todo!()
    }
}
