pub use crate::error::*;
use crate::segment::{FormatSegment, SYL_BOUND_STR, Segment, SegmentString, WORD_BOUND_STR};

/// Defines the formatting of a segment string
pub trait FormatSegmentString {
    /// Parse an ipa character segment
    fn parse(input: &str) -> Result<Self>
    where
        Self: Sized;

    /// Format as an ipa character string
    fn format(&self) -> String;
}

impl FormatSegmentString for SegmentString {
    /// constructs a segment string from a text string.
    /// - the syntax for a segment is the same as Segment::from_string.
    /// - initial and trailing whitespace is ignored.
    /// - hash (#) is interpreted as a word boundary.
    /// - ([unimplemented]) `.` is interpreted as a sylable boundary... TODO check if really unimplemented
    /// `put_word_bounds`: if true, word bounderies will be placed at the extemeties of the segment string.
    ///
    /// TODO this can be recursive
    /// TODO needs to read diacritics
    /// TODO put this in parse.rs
    fn parse(s: &str) -> Result<Self> {
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
            match s.get(start..(start + 1)) {
                None => {}
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
                if let Ok(seg) = Segment::parse_segment(&s[start..end]) {
                    seg_from_substr = Some(seg);
                    best_end = end;
                }
                end += 1;

                match s.get(start..end) {
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
                seg_str.push(seg);
                start = best_end;
                end = start + 1;
            } else {
                // if not, return error
                return Err(Error::SegmentStringParsingError(format!(
                    "Could not parse a segment out of the remaining string:\n{}",
                    s[start..s.len()].to_string()
                )));
            }
        }

        return Ok(seg_str);
    }

    fn format(&self) -> String {
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
        format!("{}", s)
    }
}
