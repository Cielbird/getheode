pub use crate::error::*;
use crate::segment::{
    Segment, SegmentString, Sylable, SYL_BOUND_CHAR, SYL_STRESS_BOUND_CHAR, WORD_BOUND_STR
};

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
    /// parses a string and identifies the sequence of segments
    /// apostrophe is used to indicate the beginning of a stressed sylable
    /// 
    /// TODO docs
    fn parse(input_str: &str) -> Result<Self> {
        // remove all whitespace
        let input: String = input_str.chars().filter(|c| !c.is_whitespace()).collect();

        let mut remaining_input: &str = &input;
        // index of the first character of the remaining input
        let mut index = 0;
        let mut result = SegmentString::new();

        let mut cur_sylable = Sylable {
            stressed: false,
        };
        let mut cur_word = ();

        while !remaining_input.is_empty() {
            // parse a sylable maker (. or ')
            let stressed_syl = remaining_input.starts_with(SYL_STRESS_BOUND_CHAR);
            let unstressed_syl = remaining_input.starts_with(SYL_BOUND_CHAR);
            let is_word = remaining_input.starts_with(WORD_BOUND_STR);
            if stressed_syl || unstressed_syl || is_word {
                // // record the last sylable
                // cur_sylable.end = index;
                // if cur_sylable.start < cur_sylable.end {
                //     result.sylables.push(cur_sylable.clone());
                // }
                // // start the next sylable
                // cur_sylable = SegmentStringSylable {
                //     start: index,
                //     end: -1,
                //     stressed: stressed_syl,
                // };

                // if is_word {
                //     // record the last word
                //     cur_word.end = index;
                //     if cur_word.start < cur_word.end {
                //         result.words.push(cur_word.clone());
                //     }
                //     // start the next word
                //     cur_word = SegmentStringWord {
                //         start: index,
                //         end: -1,
                //     };
                // }
                // remaining_input = &remaining_input[1..];
                // continue;
            }

            index += 1;

            // match the next segment
            let mut seg_from_substr = None;
            let mut end = 0;
            let mut best_end = 0;
            while end <= remaining_input.len() {
                match remaining_input.get(..end) {
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
                result.segs.push(seg);
                remaining_input = &remaining_input[best_end..];
            } else {
                // if not, return error
                return Err(Error::SegmentStringParsingError(format!(
                    "Could not parse a segment out of the remaining string:\n{}",
                    remaining_input
                )));
            }
        }

        // result.sylables.push(cur_sylable);
        // result.words.push(cur_word);

        // result.fix_sylable_boundaries();
        // result.fix_word_boundaries();
        Ok(result)
    }

    fn format(&self) -> String {
        // TODO put this in a config struct
        let show_word_bounds = true;

        let mut s: String = String::new();
        // we will remove the elements as we go
        let word_bounds = &self.words;
        let syl_bounds = &self.sylables;
        if self.segs.is_empty() {
            return "[]".to_string();
        }
        for i in 0..(self.segs.len() + 1) {
            // let is_word_bound = word_bounds
            //     .iter()
            //     .any(|x| x.start == i as isize || x.end == i as isize);
            // if is_word_bound {
            //     if show_word_bounds || i != 0 && i != self.segs.len() {
            //         s.push(WORD_BOUND_STR[1])
            //     }
            // }
            // if let Some(syl) = syl_bounds.iter().filter(|x| x.start == i as isize).next() {
            //     if syl.stressed {
            //         s.push(SYL_STRESS_BOUND_CHAR)
            //     } else if !is_word_bound {
            //         s.push(SYL_BOUND_CHAR)
            //     }
            // }
            // if i < self.segs.len() {
            //     s.push_str(&self.segs[i].to_string());
            // }
        }
        s.to_string()
    }
}
