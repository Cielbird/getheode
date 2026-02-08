use core::fmt;
use std::fmt::Display;

use crate::segment::FormatPhonologicalString;
use crate::segment::PhonologicalStringSlice;
use crate::segment::Segment;
use crate::segment::string::element::PhonologicalElement;

pub(crate) const WORD_BOUND_STR: [char; 2] = ['#', '_'];
pub(crate) const SYL_STRESS_BOUND_CHAR: char = '\'';
pub(crate) const SYL_BOUND_CHAR: char = '.';

/// a versatile struct that represents a sequence of phonological items, including segments and
/// syllable or word boundaries
///
/// can be used to represent words, parts of words, sound patterns,
/// phonological feature sequences, sentences, and phrases.
/// some vocabulary:
/// - complete: a segment string is complete when all of its segments are
///   complete, ie, entirely defined. ex: words, parts of words, phrases
/// - incomplete: a segment string is incomplete when one of its segments has
///   an undefined feature. useful for matching segments in rules.
/// - worded: when the segment has word boundaries on its extremities. ex:
///   words, phrases, etc.
///
/// for a segment string to be a word, it must be worded and complete.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PhonologicalString {
    /// Sequence of segments or boundaries
    pub(crate) elements: Vec<PhonologicalElement>,
}

impl PhonologicalString {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// Get a worded version of this segment string
    pub fn worded(mut self) -> Self {
        if self.elements.is_empty() {
            return self;
        }

        if self.elements[0] != PhonologicalElement::WordBoundary {
            if let PhonologicalElement::SyllableBoundary { stressed: _ } = self.elements[0] {
                self.elements[0] = PhonologicalElement::WordBoundary;
            }
            self.elements.insert(0, PhonologicalElement::WordBoundary);
        }

        let last = self.elements.last().unwrap().clone();
        if last != PhonologicalElement::WordBoundary {
            if let PhonologicalElement::SyllableBoundary { stressed: _ } = last {
                *self.elements.last_mut().unwrap() = PhonologicalElement::WordBoundary;
            }
            self.elements.push(PhonologicalElement::WordBoundary);
        }

        self
    }

    /// Create an unworded string of segments
    pub fn from_segments<S>(segments: S) -> Self
    where
        S: Into<Vec<Segment>>,
    {
        let segments = segments.into();

        let mut items = vec![];
        for s in segments {
            items.push(PhonologicalElement::SegmentElement(s));
        }

        Self { elements: items }
    }

    pub fn from_elements<E>(elements: E) -> Self
    where
        E: Into<Vec<PhonologicalElement>>,
    {
        Self {
            elements: elements.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        // call implementation in slice struct
        self.slice_all().is_complete()
    }

    /// Replace a part of a phonological string
    ///
    /// - `start`, `end` : bounds of replacement, including both segments and syllable/word bounds
    pub fn replace(&mut self, start: usize, end: usize, mut replacement: PhonologicalString) {
        self.elements.drain(start..end);

        for (i, seg) in replacement.elements.drain(..).enumerate() {
            self.elements.insert(start + i, seg);
        }
    }

    pub fn slice_all<'a>(&'a self) -> PhonologicalStringSlice<'a> {
        self.slice(0, self.elements.len())
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> PhonologicalStringSlice<'a> {
        PhonologicalStringSlice::new(self, start, end)
    }

    /// count number of segments in the string
    pub fn segments_len(&self) -> usize {
        self.elements
            .iter()
            .filter(|e| matches!(e, PhonologicalElement::SegmentElement(_)))
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// Length of elements (sounds and boundary markers) in the string
    pub(crate) fn element_len(&self) -> usize {
        self.elements.len()
    }
}

impl Display for PhonologicalString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Default for PhonologicalString {
    fn default() -> Self {
        Self::new()
    }
}
