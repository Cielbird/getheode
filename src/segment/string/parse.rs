pub use crate::error::*;
use crate::segment::{
    PhonologicalString, SYL_BOUND_CHAR, SYL_STRESS_BOUND_CHAR, Segment, WORD_BOUND_STR,
};

use super::element::PhonologicalElement;

/// Defines the formatting of a segment string
pub trait FormatPhonologicalString {
    /// Parse an ipa character segment
    fn parse(input: &str) -> Result<Self>
    where
        Self: Sized;

    /// Format as an ipa character string
    fn format(&self) -> String;
}

impl FormatPhonologicalString for PhonologicalString {
    /// parses a string and identifies the sequence of segments
    /// apostrophe is used to indicate the beginning of a stressed syllable
    ///
    /// TODO docs
    fn parse(input_str: &str) -> Result<Self> {
        // remove all whitespace
        let input: String = input_str.chars().filter(|c| !c.is_whitespace()).collect();

        // parse string from the beginning
        let mut remaining_input: &str = &input;

        let mut result = PhonologicalString::new();

        while !remaining_input.is_empty() {
            // parse a sylable maker (. or ')
            if remaining_input.starts_with(SYL_STRESS_BOUND_CHAR) {
                result
                    .elements
                    .push(PhonologicalElement::SyllableBoundary { stressed: true });
                remaining_input = &remaining_input[1..];
                continue;
            }
            if remaining_input.starts_with(SYL_BOUND_CHAR) {
                result
                    .elements
                    .push(PhonologicalElement::SyllableBoundary { stressed: false });
                remaining_input = &remaining_input[1..];
                continue;
            }
            // parse a word maker
            if remaining_input.starts_with(WORD_BOUND_STR) {
                result.elements.push(PhonologicalElement::WordBoundary);
                remaining_input = &remaining_input[1..];
                continue;
            }

            // match the next segment
            let mut seg_from_substr = None;
            let mut end = 0;
            let mut best_end = 0;
            while end <= remaining_input.len() {
                match remaining_input.get(..end) {
                    None => {
                        end += 1;
                        continue;
                    }
                    Some(segs_str) => {
                        if let Ok(seg) = Segment::parse_segment(segs_str) {
                            seg_from_substr = Some(seg);
                            best_end = end;
                        }
                        end += 1;
                    }
                }
            }

            // if we found a substring that forms a segment, add it to the segment string
            if let Some(seg) = seg_from_substr {
                result.elements.push(PhonologicalElement::SegmentElement(seg));
                remaining_input = &remaining_input[best_end..];
            } else {
                // if not, return error
                return Err(Error::SegmentStringParsingError(format!(
                    "Could not parse a segment out of the remaining string:\n{}",
                    remaining_input
                )));
            }
        }

        Ok(result)
    }

    fn format(&self) -> String {
        if self.elements.is_empty() {
            return "[]".to_string();
        }

        let mut s: String = String::new();
        for i in &self.elements {
            s.push_str(&format_phonological_element(i));
        }

        s
    }
}

// TODO add config options
fn format_phonological_element(i: &PhonologicalElement) -> String {
    match i {
        PhonologicalElement::SegmentElement(segment) => segment.to_string(),
        PhonologicalElement::SyllableBoundary { stressed } => {
            if *stressed {
                SYL_STRESS_BOUND_CHAR.to_string()
            } else {
                SYL_BOUND_CHAR.to_string()
            }
        }
        PhonologicalElement::WordBoundary => WORD_BOUND_STR[1].to_string(),
    }
}
