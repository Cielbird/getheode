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
        let mut seg_start: usize = 0;
        let mut seg_end: usize = 1;
        
        loop {
            let seg_from_substr;
            loop {
                if seg_end > s.len() {
                    return Err(SegmentStringParsingError(s[seg_start..s.len()].to_string()));
                }
                if let Ok(seg) = Segment::from_string(&s[seg_start..seg_end]) {
                    seg_from_substr = seg; 
                    break;
                }
                seg_end += 1;
            }
            // seg_start and seg_end delimit a valid Segment
            seg_str.push(seg_from_substr);
            // have we reached the end of the string?
            if seg_end == s.len() {
                break;
            }
            seg_start = seg_end;
            seg_end = seg_start + 1;
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

