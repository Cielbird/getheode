use core::fmt;
use std::fmt::Display;

use crate::segment::bounds::ChunkSequence;
use crate::segment::FormatSegmentString;
use crate::segment::Segment;
use crate::segment::SegmentStringSlice;
use crate::segment::Sylable;

pub(crate) const WORD_BOUND_STR: [char; 2] = ['#', '_'];
pub(crate) const SYL_STRESS_BOUND_CHAR: char = '\'';
pub(crate) const SYL_BOUND_CHAR: char = '.';

/// a versatile struct that represents a sequence of phonological segments, and can
/// indicate word and sylable boundaries.
///
/// can be used to represent words, parts of words, sound patterns,
/// phonological feature sequences, sentences, and phrases.
/// some vocabulary:
/// - complete: a segment string is complete when all of its segments are
///   complete, ie, entirely defined. ex: words, parts of words, phrases
/// - incomplete: a segment string is incomplete when one of its segments has
///   an undefined feature. useful for matching segments in rules.
/// - worded: when the segment has word bounaries on its extremeties. ex:
///   words, phrases, etc.
///
/// for a segment string to be a word, it must be worded and complete.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SegmentString {
    /// segments that exist in this string of segments
    pub(crate) segs: Vec<Segment>,

    /// indices of word boundaries w/ respect to the segment vector.
    /// given an element `i` the actual boundary is found between segment at `i` and
    /// segment at `i-1`
    /// ordered in ascending order.
    /// the first and last element must be 0 and segs.len() respectively,
    /// representing the start and end of the string.
    pub(crate) words: ChunkSequence<()>,

    /// Sylables included in this string. Indices are relative to the first element in the
    /// segments list
    pub(crate) sylables: ChunkSequence<Sylable>,
}

impl SegmentString {
    pub fn new() -> Self {
        Self {
            segs: Vec::new(),
            words: ChunkSequence::single_unbounded(0),
            sylables: ChunkSequence::single_unbounded(0),
        }
    }

    /// Get a worded version of this segment string
    pub fn worded(mut self) -> SegmentString {
        self.words = self.words.bounded();
        self
    }

    /// Create an unworded string of segments
    pub fn from_segments<S>(segments: S) -> Self where S: Into<Vec<Segment>> {
        let segments = segments.into();
        let len = segments.len();
        Self {
            segs: segments,
            words: ChunkSequence::single_unbounded(len),
            sylables: ChunkSequence::single_unbounded(len),
        }
    }

    pub fn is_complete(&self) -> bool {
        // call implementation in slice struct
        self.slice_all().is_complete()
    }

    /// does the pattern match this segment string at the given position
    /// the features defined in the pattern must be defined the same in the string.
    pub fn is_match(&self, pattern: &SegmentString, pos: usize) -> bool {
        // call implementation in slice struct
        self.slice_all().is_match(pattern, pos)
    }

    pub fn push(&mut self, seg: Segment) {
        self.segs.push(seg);
        self.words.extend_last(1);
        self.sylables.extend_last(1);
    }

    /// moves the items of `str` to this seg string.  
    pub fn append(&mut self, mut str: SegmentString) {
        self.segs.append(&mut str.segs);
    }

    pub fn replace(&mut self, start: usize, end: usize, mut replacement: SegmentString) {
        // How did segments after the replacement shift
        let shift = replacement.len() as isize - end as isize + start as isize;
        println!("new end: {shift}");
        self.segs.drain(start..end);

        for (i, seg) in replacement.segs.drain(..).enumerate() {
            self.segs.insert(start + i, seg);
        }

        self.words.replace(start, end, replacement.words);
        self.sylables.replace(start, end, replacement.sylables);
    }

    // alternative to getting a one segment slice
    pub fn get_segment(&self, i: usize) -> &Segment {
        &self.segs[i]
    }

    pub fn slice_all<'a>(&'a self) -> SegmentStringSlice<'a> {
        self.slice(0, self.len())
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> SegmentStringSlice<'a> {
        SegmentStringSlice::new(&self, start, end)
    }

    pub fn len(&self) -> usize {
        self.segs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.segs.is_empty()
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for SegmentString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Default for SegmentString {
    fn default() -> Self {
        Self::new()
    }
}
