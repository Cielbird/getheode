use crate::segment::{PhonologicalString, string::element::PhonologicalElement};

/// a borrowed type (reference) of a section of an existing [PhonologicalString] type
/// do not use this if you plan to modify the referenced struct.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct PhonologicalStringSlice<'a> {
    /// the segement string this refers to
    pub(crate) string: &'a PhonologicalString,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl<'x> PhonologicalStringSlice<'x> {
    pub fn new<'a>(string: &'a PhonologicalString, start: usize, end: usize) -> PhonologicalStringSlice<'a> {
        assert!(end > start);
        assert!(end <= string.segments_len());

        PhonologicalStringSlice::<'a> { string, start, end }
    }

    /// returns true if the segment slice is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// internal implementation of is_complete so that the SegmentStringSlice reference
    ///     type can use the same code.
    pub fn is_complete(&self) -> bool {
        for seg in &self.string.elements[self.start..self.end] {
            if let PhonologicalElement::SegmentElement(seg) = seg
                && !seg.is_complete()
            {
                return false;
            }
        }
        true
    }

    pub fn slice_all<'a>(&'a self) -> PhonologicalStringSlice<'a> {
        *self
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> PhonologicalStringSlice<'a> {
        Self::new(self.string, self.start + start, self.start + end)
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
