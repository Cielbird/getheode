use core::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::errors::GetheodeError;
use crate::segment::Segment;
use crate::errors::GetheodeError::SegmentStringParsingError;

#[derive(Debug, Clone)]
pub struct SegmentString(Vec<Segment>);

impl SegmentString {
    pub fn new() -> Self {
        return Self(Vec::new());
    }

    pub fn from_string(s: &str) -> Result<Self, GetheodeError> {// todo make a error enum for this library 
        // seg string we will return
        let mut seg_str = SegmentString::new();
        let mut start = 0;
        let mut end = 0;
        let mut best_end = 0;
        // where we keep the best match
        let mut seg_from_substr;

        loop {
            // if is Some, we'll keep looking because we want the largest match possible
            seg_from_substr = None;
            while end <= s.len() {
                // don't try to parse when we're in the middle of a utf8 char, not on a boundary
                if !s.is_char_boundary(end) {
                    end += 1;
                    continue;
                }
                if let Ok(seg) = Segment::from_string(&s[start..end]) {
                    seg_from_substr = Some(seg);
                    best_end = end;
                }
                end += 1;
            }
            // if we found a substring that forms a segment, add it to the segment string
            if let Some(seg) = seg_from_substr {
                seg_str.push(seg);
                // have we reached the end of the string?
                if best_end == s.len() {
                    break;
                }
                start = best_end;
                end = start + 1;
            } else {
                // if not, return error
                return Err(SegmentStringParsingError(s[start..s.len()].to_string()));
            }
        }
        return Ok(seg_str);
    }
    
    pub fn from_segments(segments: Vec<Segment>) -> Self {
        return Self(segments);
    }
}

// allow SegmentString to act like a Vec<Segment>
impl Deref for SegmentString {
    type Target = Vec<Segment>;

    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}

// mutable operations on SegmentString
impl DerefMut for SegmentString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}

impl FromIterator<Segment> for SegmentString {
    fn from_iter<I: IntoIterator<Item = Segment>>(iter : I) -> Self {
        let mut string = SegmentString::new();
        for item in iter {
            string.push(item);
        }
        return string;
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for SegmentString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s: String = String::new();
        for seg in &self.0 {
            s.push_str(&seg.to_string());
        }
        write!(f, "{}", s)
    }
}

