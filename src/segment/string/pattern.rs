use core::fmt;
use std::fmt::Display;

use crate::error::*;
use crate::segment::{FormatPhonologicalString, PhonologicalString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhonologicalStringPattern {
    string: PhonologicalString,
}

impl PhonologicalStringPattern {
    /// does the pattern match this segment string at the given position
    /// returns true if the pattern matches the segments and boundaries at position `pos`
    /// returns false otherwise.
    ///
    /// the features defined in the pattern must be defined the same in the string.
    pub fn is_match(&self, haystack: &PhonologicalString, pos: usize) -> bool {
        // TODO return a list of matches, with start and end positions
        if pos + self.string.elements.len() > haystack.elements.len() {
            return false;
        }
        for (i, pattern_seg) in self.string.elements.iter().enumerate() {
            let seg = &haystack.elements[i + pos];
            if !seg.matches(pattern_seg) {
                return false;
            }
        }

        true
    }

    /// Length of elements (sounds and boundary markers) in the pattern
    pub fn element_len(&self) -> usize {
        self.string.element_len()
    }
    
    pub(crate) fn segment_len(&self) -> usize {
        self.string.segments_len()
    }
}

impl From<PhonologicalString> for PhonologicalStringPattern {
    fn from(string: PhonologicalString) -> Self {
        Self { string }
    }
}

impl FormatPhonologicalString for PhonologicalStringPattern {
    fn parse(input_str: &str) -> Result<Self> {
        Ok(Self {
            string: PhonologicalString::parse(input_str)?,
        })
    }

    fn format(&self) -> String {
        self.string.format()
    }
}

impl Display for PhonologicalStringPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}
