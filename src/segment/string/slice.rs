use crate::segment::SegmentString;

/// a borrowed type (reference) of a section of an existing SegmentString type
/// do not use this if you plan to modify the referenced struct.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SegmentStringSlice<'a> {
    /// the segement string this refers to
    pub(crate) string: &'a SegmentString,
    pub(crate) start: usize,
    pub(crate) end: usize,
}

// allow SegmentString to act like a Vec<Segment>
impl<'x> SegmentStringSlice<'x> {
    pub fn new<'a>(string: &'a SegmentString, start: usize, end: usize) -> SegmentStringSlice<'a> {
        assert!(end > start);
        assert!(end <= string.segs.len());

        SegmentStringSlice::<'a> { string, start, end }
    }

    /// returns true if the segment slice is empty
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// internal implementation of is_complete so that the SegmentStringSlice reference
    ///     type can use the same code.
    pub fn is_complete(&self) -> bool {
        for seg in &self.string.segs[self.start..self.end] {
            if !seg.is_complete() {
                return false;
            }
        }
        true
    }

    pub fn slice_all<'a>(&'a self) -> SegmentStringSlice<'a> {
        *self
    }

    /// get a slice reference of a segment string
    pub fn slice<'a>(&'a self, start: usize, end: usize) -> SegmentStringSlice<'a> {
        Self::new(self.string, self.start + start, self.start + end)
    }

    /// does the pattern match this segment string at the given position
    /// returns true if the pattern matches the segments and boundaries at position `pos`
    /// returns false otherwise.
    ///
    /// the features defined in the pattern must be defined the same in the string.
    pub fn is_match(&self, pattern: &SegmentString, pos: usize) -> bool {
        if pos + pattern.len() > self.string.segs.len() {
            return false;
        }
        for (i, pattern_seg) in pattern.segs.iter().enumerate() {
            let seg = &self.string.segs[i + pos];
            if !seg.matches(pattern_seg) {
                return false;
            }
        }
        let offset = (self.start + pos) as isize;

        // Check word and sylable bounds
        if !self.string.words.is_match(&pattern.words, offset) {
            return false;
        }
        // if !self.string.sylables.is_match(&pattern.sylables, offset) {
        //     return false;
        // }

        true
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}
