use core::fmt;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

use crate::error::Error;
use crate::error::Result;
use crate::error::Error::SegmentStringParsingError;
use crate::segment::Segment;

const WORD_BOUND_STR: &str = "#";
const SYL_BOUND_STR: &str = ".";

/// a versatile struct that represents a sequence of phonological segments, and can
/// indicate word and sylable boundaries.
/// 
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
#[derive(Debug, Clone, Eq, PartialEq)]
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
    syl_boundaries: Vec<usize>,
}

impl SegmentString {
    pub fn empty() -> Self {
        return Self {
            segs: Vec::new(),
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new(),
        };
    }

    /// constructs a segment string from a text string.
    /// - the syntax for a segment is the same as Segment::from_string.
    /// - initial and trailing whitespace is ignored.
    /// - whitespace between segments is interpreted as a word boundary.
    /// - ([unimplemented]) `.` is interpreted as a sylable boundary... TODO check if really unimplemented
    /// `put_word_bounds`: if true, word bounderies will be placed at the extemeties of the segment string.
    /// 
    /// TODO this can be recursive
    /// TODO needs to read diacritics
    pub fn new(s: &str) -> Result<Self> {
        // seg string we will return
        let mut seg_str = Self {
            segs: Vec::new(),
            // start off with initial word boundary
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new(),
        };
        let mut start = 0;
        let mut end = 1;
        let mut best_end = 1;
        // where we keep the best match
        let mut seg_from_substr;

        while start < s.len() {
            // if starting char is a space, interpret as a word bounary and cont
            // look at next byte, if it is a char, examine it, otherwise move on.
            match s.get(start..(start+1)) {
                None => {},
                Some(char_str) => {
                    if char_str == WORD_BOUND_STR {
                        // is the word boundary at the same spot as another sylable boundary?
                        let bounds = &seg_str.syl_boundaries;
                        if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                            return Err(Error::SegmentStringParsingError(format!(
                                "cannot add a sylable boundary and a word boundary in the same spot: {}",
                                s[0..(start + 1)].to_string()
                            )));
                        }
                        // word boundary?
                        let bounds = &mut seg_str.word_boundaries;
                        if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                            return Err(Error::SegmentStringParsingError(format!(
                                "cannot have multiple word boundaries in the same spot: {}",
                                s[0..(start + 1)].to_string()
                            )));
                        }
        
                        bounds.push(seg_str.segs.len());
                        start = start + 1;
                        end = start + 1;
                        continue;
                    } else if char_str == SYL_BOUND_STR {
                        // is the sylable boundary at the same spot as another word boundary?
                        let bounds = &seg_str.word_boundaries;
                        if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                            return Err(Error::SegmentStringParsingError(format!(
                                "cannot add a sylable boundary and a word boundary in the same spot: {}",
                                s[0..(start + 1)].to_string()
                            )));
                        }
                        // sylable boundary?
                        let bounds = &mut seg_str.syl_boundaries;
                        if bounds.len() > 0 && bounds[bounds.len() - 1] == seg_str.segs.len() {
                            return Err(Error::SegmentStringParsingError(format!(
                                "cannot have multiple sylable boundaries in the same spot: {}",
                                s[0..(start + 1)].to_string()
                            )));
                        }
                        bounds.push(seg_str.segs.len());
                        start = start + 1;
                        end = start + 1;
                        continue;
                    }
                }
            }

            // we want the largest match possible
            seg_from_substr = None;
            while end <= s.len() {
                // don't try to parse when we're in the middle of a utf8 char, not on a boundary
                // or if the last char in our cur range is a space
                if !s.is_char_boundary(end) {
                    end += 1;
                    continue;
                }
                if let Ok(seg) = Segment::from_string(&s[start..end]) {
                    seg_from_substr = Some(seg);
                    best_end = end;
                }
                end += 1;

                match s.get(start..end) {
                    None => {
                        end += 1;
                        continue;
                    },
                    Some(segs_str) => {
                        if let Ok(seg) = Segment::from_string(segs_str) {
                            seg_from_substr = Some(seg);
                            best_end = end;
                        }
                        end += 1;
                    }
                }
            }

            // if we found a substring that forms a segment, add it to the segment string
            if let Some(seg) = seg_from_substr {
                seg_str.push(seg);
                start = best_end;
                end = start + 1;
            } else {
                // if not, return error
                return Err(SegmentStringParsingError(format!(
                    "Could not parse a segment out of the remaining string:\n{}",
                    s[start..s.len()].to_string()
                )));
            }
        }

        return Ok(seg_str);
    }

    pub fn to_worded(mut seg_str: SegmentString) -> SegmentString {
        // don't make duplicate boundaries
        let bounds = &mut seg_str.word_boundaries;
        if bounds.len() == 0 || bounds[0] != 0 {
            bounds.insert(0, 0);
        }

        let last_word_bound = bounds[bounds.len() - 1];
        if last_word_bound != seg_str.segs.len() {
            bounds.push(seg_str.segs.len());
        }
        return seg_str;
    }

    pub fn from_segments(segments: Vec<Segment>) -> Self {
        return Self {
            segs: segments,
            word_boundaries: Vec::new(),
            syl_boundaries: Vec::new(),
        };
    }

    pub fn is_complete(&self) -> bool {
        // call implementation in slice struct
        return self.slice_all().is_complete();
    }

    /// does the pattern match this segment string at the given position
    pub fn is_match(&self, pattern: &SegmentString, pos: usize) -> bool {
        // call implementation in slice struct
        return self.slice_all().is_match(pattern, pos);
    }

    pub fn push(&mut self, seg: Segment) {
        self.segs.push(seg);
        if self.word_boundaries.len() == 0 {
            return;
        }
        // update trailing word boundary
        let bounds = &mut self.word_boundaries;
        if bounds.len() > 0 && bounds[bounds.len() - 1] == self.segs.len() {
            let i = bounds.len() - 1;
            bounds[i] += 1;
        }
    }

    /// moves the items of `str` to this seg string.  
    pub fn append(&mut self, mut str: SegmentString) {
        self.segs.append(&mut str.segs);
    }

    pub fn replace(&mut self, start: usize, end: usize, replacement: &SegmentString) {
        self.segs.drain(start..end);

        for i in 0..replacement.len() {
            self.segs.insert(start + i, replacement[i].clone());
        }
        // update any word boundaries that come after
        let mut i = 0;
        let bounds = &mut self.word_boundaries;
        while i < bounds.len() {
            if bounds[i] <= start {
                i += 1;
                continue;
            } else if bounds[i] < end {
                bounds.remove(i);
                println!("removed at {}", i);
            } else {
                // otherwise, adjust as
                let offset: isize = start as isize - end as isize + replacement.len() as isize;
                bounds[i] = bounds[i].saturating_add_signed(offset);
                i += 1;
            }
        }
        // update any syl boundaries that come after
        i = 0;
        let bounds = &mut self.syl_boundaries;
        while i < bounds.len() {
            if bounds[i] <= start {
                i += 1;
                continue;
            } else if bounds[i] < end {
                bounds.remove(i);
                println!("removed at {}", i);
            } else {
                // otherwise, adjust as
                bounds[i] += start + replacement.len() - end;
                i += 1;
            }
        }
    }

    // alternative to getting a one segment slice
    pub fn get_segment(&self, i: usize) -> &Segment {
        return &self.segs[i];
    }

    pub fn slice_all<'a>(&'a self) -> SegmentStringSlice<'a> {
        return self.slice(0, self.len());
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> SegmentStringSlice<'a> {
        // get the word boundary indices that are in the range we want
        let word_bounds_start = self.word_boundaries.iter()
            .position(|x| x >= &start)
            .unwrap_or(0);
        let word_bounds_end = self.word_boundaries.iter()
            .enumerate()
            .filter(|&(_i, &bound_index)| bound_index <= end)
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);

        // get the sylable boundary indices that are in the range we want
        let syl_bounds_start = self.syl_boundaries.iter()
            .position(|x| x >= &start)
            .unwrap_or(0);
        let syl_bounds_end = self.syl_boundaries.iter()
            .enumerate()
            .filter(|&(_i, &bound_index)| bound_index <= end)
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);

        return SegmentStringSlice {
            segs: &self.segs[start..end],
            offset: start,
            word_boundaries: &self.word_boundaries[word_bounds_start..word_bounds_end],
            syl_boundaries: &self.syl_boundaries[syl_bounds_start..syl_bounds_end],
        };
    }

    pub fn len(&self) -> usize {
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

// allow SegmentString to act like a Vec<Segment>
impl DerefMut for SegmentString {
    fn deref_mut(&mut self) -> &mut Vec<Segment> {
        return &mut self.segs;
    }
}

impl FromIterator<Segment> for SegmentString {
    fn from_iter<I: IntoIterator<Item = Segment>>(iter: I) -> Self {
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
        let show_edge_bounds = true;
        let mut s: String = String::new();
        // we will remove the elements as we go
        let mut word_bounds = self.word_boundaries.clone();
        let mut syl_bounds = self.syl_boundaries.clone();
        for i in 0..(self.segs.len() + 1) {
            if word_bounds.len() > 0 && i == word_bounds[0] {
                word_bounds.remove(0);
                if show_edge_bounds || i != 0 && i != self.segs.len() {
                    s.push_str(WORD_BOUND_STR)
                }
            }
            if syl_bounds.len() > 0 && i == syl_bounds[0] {
                syl_bounds.remove(0);
                if show_edge_bounds || i != 0 && i != self.segs.len() {
                    s.push_str(SYL_BOUND_STR)
                }
            }
            if i < self.segs.len() {
                s.push_str(&self.segs[i].to_string());
            }
        }
        write!(f, "{}", s)
    }
}



/// a borrowed type (reference) of a section of an existing SegmentString type 
/// do not use this if you plan to modify the referenced struct.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SegmentStringSlice<'a> {
    /// the segement string this refers to
    segs: &'a [Segment],
    /// where does this slice start in the og string?
    offset: usize,
    /// positions of word boundaries in the segments (- the offset)
    word_boundaries: &'a [usize],
    /// positions of sylable boundaries in the segments (- the offset)
    syl_boundaries: &'a [usize]
}


// allow SegmentString to act like a Vec<Segment>
impl<'x> SegmentStringSlice<'x> {

    /// returns true if the segment slice is empty
    pub fn is_empty(&self) -> bool {
        return self.segs.len() == 0;
    }

    /// internal implementation of is_complete so that the SegmentStringSlice reference
    ///     type can use the same code.
    pub fn is_complete(&self) -> bool {
        for seg in self.segs {
            if !seg.is_complete() {
                return false;
            }
        }
        return true;
    }

    pub fn slice_all<'a>(&'a self) -> SegmentStringSlice<'a> {
        return *self;
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> SegmentStringSlice<'a> {
        // get the word boundary indices that are in the range we want
        let word_bounds_start = self.word_boundaries.iter()
            .position(|x| x >= &start)
            .unwrap_or(0);
        let word_bounds_end = self.word_boundaries.iter()
            .position(|x| x < &end)
            .unwrap_or(0);

        // get the sylable boundary indices that are in the range we want
        let syl_bounds_start = self.syl_boundaries.iter()
            .position(|x| x >= &start)
            .unwrap_or(0);
        let syl_bounds_end = self.syl_boundaries.iter()
            .position(|x| x < &end)
            .unwrap_or(0);

        return SegmentStringSlice {
            segs: &self.segs[start..end],
            offset: start,
            word_boundaries: &self.word_boundaries[word_bounds_start..word_bounds_end],
            syl_boundaries: &self.syl_boundaries[syl_bounds_start..syl_bounds_end],
        };
    }

    /// does the pattern match this segment string at the given position
    /// returns true if the pattern matches the segments and boundaries at position `pos`
    /// returns false otherwise.
    pub fn is_match(&self, pattern: &SegmentString, pos: usize) -> bool {
        if pos + pattern.len() > self.segs.len() {
            return false;
        }
        for (i, pattern_seg) in pattern.segs.iter().enumerate() {
            let seg = &self.segs[i + pos];
            if !seg.matches(pattern_seg) {
                return false;
            }
        }
        for word_bound in &pattern.word_boundaries {
            if !self.word_boundaries.contains(&(word_bound + &self.offset + pos)) {
                return false;
            }
        }
        for syl_bound in &pattern.syl_boundaries {
            if !self.syl_boundaries.contains(&(syl_bound + &self.offset + pos)) {
                return false;
            }
        }
        return true;
    }

    pub fn len(&self) -> usize {
        return self.segs.len();
    }
}
