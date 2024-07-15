use core::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::RangeBounds;

use crate::errors::GetheodeError;
use crate::segment::Segment;
use crate::errors::GetheodeError::SegmentStringParsingError;

const WORD_BOUND_STR: &str = "#";
const SYL_BOUND_STR: &str = ".";

/// a versatile struct that represents a sequence of phonological segments, and can 
/// indicate word and sylable boundaries.
/// can be used to represent words, parts of words, sound patterns, 
/// phonological feature sequences, sentences, and phrases.
/// some vocabulary:
/// - complete: a segment string is complete when all of its segments are 
///     complete, ie, entirely defined. ex: words, parts of words, phrases
/// - incomplete: a segment string is incomplete when one of its segments has
///     an undefined feature. useful for matching segments in rules.
/// - worded: when the segment has word bounaries on its extremeties. ex:
///     words, phrases, etc. 
/// 
/// for a segment string to be a word, it must be worded and complete.
#[derive(Debug, Clone)]
pub struct SegmentString {
    /// segments that exist in this string of segments
    segs: Vec<Segment>,

    /// indices of word boundaries w/ respect to the segment vector. 
    /// given an element `i` the actual boundary is found between segment at `i` and 
    /// segment at `i-1`
    /// ordered in ascending order.
    /// the first and last element must be 0 and segs.len() respectively,
    /// representing the start and end of the string.
    word_boundaries: Vec<usize>,

    /// indices of sylable boundaries w/ respect to the segment vector. 
    /// given an element `i` the actual boundary is found between segment at `i` and 
    /// segment at `i-1`  
    /// ordered in ascending order
    /// word boundaries are sylable boundaries. to avoid redundancy, 
    /// sylable boundaries cannot have the same index as a word boundary. 
    /// all sylable boundaries should be considered optional data. there is no objective way of
    /// delimiting sylables, it depends on the language.
    syl_boundaries: Vec<usize>
}

impl SegmentString {
    pub fn empty() -> Self {
        return Self {
            segs: Vec::new(),
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new()
        };
    }

    /// constructs a segment string from a text string 
    /// that has word bounds at the beginning and end.
    /// could be one or multiple words.
    pub fn new_worded(s: &str) -> Result<Self, GetheodeError> {// todo make a error enum for this library 
        match Self::new(s) {
            Err(e) => Err(e),
            Ok(mut seg_str) => {
                // don't make duplicate boundaries
                let bounds = &mut seg_str.word_boundaries;
                if bounds.len() == 0 || bounds[0] != seg_str.segs.len() {
                    bounds.push(seg_str.segs.len());
                }

                let last_word_bound = bounds[bounds.len() - 1];
                if last_word_bound != seg_str.segs.len() {
                    bounds.push(seg_str.segs.len());
                }

                return Ok(seg_str);
            }
        }
    }

    /// constructs a segment string from a text string.
    /// - the syntax for a segment is the same as Segment::from_string. 
    /// - initial and trailing whitespace is ignored. 
    /// - whitespace between segments is interpreted as a word boundary. 
    /// - ([unimplemented]) `.` is interpreted as a sylable boundary 
    /// `put_word_bounds`: if true, word bounderies will be placed at the extemeties of the segment string.
    pub fn new(s: &str) -> Result<Self, GetheodeError> {// todo make a error enum for this library 
        // seg string we will return
        let mut seg_str = Self {
            segs: Vec::new(),
            // start off with initial word boundary
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new()
        };
        let mut start = 0;
        let mut end = 1;
        let mut best_end = 1;
        // where we keep the best match
        let mut seg_from_substr;

        while start < s.len() {
            // if starting char is a space, interpret as a word bounary and cont
            if s[start..(start+1)].eq(WORD_BOUND_STR) {
                // is the word boundary at the same spot as another sylable boundary?
                let bounds = &seg_str.syl_boundaries;
                if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                    return Err(GetheodeError::SegmentStringParsingError(s[0..(start+1)].to_string()));
                }
                // word boundary?
                let bounds = &mut seg_str.word_boundaries;
                if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                    return Err(GetheodeError::SegmentStringParsingError(s[0..(start+1)].to_string()));
                }
                
                bounds.push(seg_str.segs.len());
                start = start + 1;
                end = start + 1;
                continue;
            }
            
            // if starting char is a period `.`, interpret as a sylable bounary and cont
            if s[start..(start+1)].eq(SYL_BOUND_STR) {
                // is the sylable boundary at the same spot as another word boundary?
                let bounds = &seg_str.word_boundaries;
                if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                    return Err(GetheodeError::SegmentStringParsingError(s[0..(start+1)].to_string()));
                }
                // sylable boundary?
                let bounds = &mut seg_str.syl_boundaries;
                if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                    return Err(GetheodeError::SegmentStringParsingError(s[0..(start+1)].to_string()));
                }
                bounds.push(seg_str.segs.len());
                start = start + 1;
                end = start + 1;
                continue;
            }

            // we want the largest match possible
            seg_from_substr = None;
            while end <= s.len() {
                // don't try to parse when we're in the middle of a utf8 char, not on a boundary
                // or if the last char in our cur range is a space
                if !s.is_char_boundary(end){
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
        return Self {
            segs: segments, 
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new()
        }
    }

    pub fn is_complete(&self) -> bool {
        for seg in &self.segs {
            if !seg.is_complete() {
                return false;
            }
        }
        return true;
    }

    /// does the pattern match this segment string at the given position
    pub fn is_match(&self, pattern: &SegmentString, pos: usize) -> bool {
        for (i, pattern_seg) in pattern.segs.iter().enumerate() {
            let seg = &self[pos + i];
            if !seg.matches(pattern_seg) {
                return false;
            }
        }
        return true;
    }

    /// finds the start positions of matches where this segment string
    /// matches the other segment.
    pub fn find_matches(&self, pattern: &SegmentString, start: usize, end:usize) -> Vec<usize> {
        let mut matches = vec![];
        for i in start..end {
            if self.is_match(pattern, i) {
                matches.push(i);
            }
        }
        return matches;
    }

    pub fn push(&mut self, seg: Segment) {
        self.segs.push(seg);
        if self.word_boundaries.len() == 0 {
            return;
        }
        // update trailing word boundary
        let bounds = &mut self.word_boundaries;
        if bounds.len() > 0 && bounds[bounds.len()-1] == self.segs.len() {
            let i = bounds.len()-1;
            bounds[i] += 1;
        }
    }

    pub fn replace<R>(&mut self, range: R, replacement: &SegmentString)
    where // I have no idea why this works, I just copied Vec::drain.
        R: RangeBounds<usize> + Clone,
    {
        self.segs.drain(range.clone());
        let start;
        match range.start_bound() {
            std::ops::Bound::Unbounded => start = 0 as usize,
            std::ops::Bound::Included(i) => start = *i,
            std::ops::Bound::Excluded(i) => start = i + 1
        }
        for i in 0..replacement.len() {
            self.segs.insert(start + i, replacement[i].clone());
        }
        // update trailing word boundary
        let bounds = &mut self.word_boundaries;
        if bounds.len() > 0 && bounds[bounds.len()-1] == self.segs.len() {
            let i = bounds.len()-1;
            bounds[i] += 1;
        }
    }

    pub fn len(&self) -> usize{
        return self.segs.len();
    }
}

// allow SegmentString to act like a Vec<Segment>
impl Deref for SegmentString {
    type Target = Vec<Segment>;

    fn deref(&self) -> &Self::Target {
        return &self.segs;
    }
}

impl Index<usize> for SegmentString {
    type Output = Segment;
    /// for easy indexing 
    fn index(&self, i: usize) -> &Self::Output {
        return &self.segs[i];
    }
}

impl IndexMut<usize> for SegmentString {
    /// for easy indexing when mutating the string
    fn index_mut(&mut self, i: usize) -> &mut Segment {
        return &mut self.segs[i];
    }
}

impl FromIterator<Segment> for SegmentString {
    fn from_iter<I: IntoIterator<Item = Segment>>(iter : I) -> Self {
        let mut string = SegmentString::empty();
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
        // we will remove the elements as we go
        let mut word_bounds = self.word_boundaries.clone();
        let mut syl_bounds = self.syl_boundaries.clone();
        for i in 0..(self.segs.len()+1) {
            if word_bounds.len() > 0 && i==word_bounds[0] {
                word_bounds.remove(0);
                s.push_str(WORD_BOUND_STR)
            }
            if syl_bounds.len() > 0 && i==syl_bounds[0] {
                syl_bounds.remove(0);
                s.push_str(SYL_BOUND_STR)
            }
            if i < self.segs.len() {
                s.push_str(&self.segs[i].to_string());
            }
        }
        write!(f, "{}", s)
    }
}
