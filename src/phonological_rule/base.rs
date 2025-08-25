use crate::{error::Result, segment::SegmentString};

#[derive(Debug)]
pub struct PhonologicalRule {
    pub input_opts: Vec<SegmentString>,
    pub output: SegmentString,
    pub pre_context_opts: Vec<SegmentString>,
    pub post_context_opts: Vec<SegmentString>,
}

impl PhonologicalRule {
    pub fn apply(&self, s: &SegmentString) -> Result<SegmentString> {
        // string we will be modifying and returning
        let mut string = s.clone();

        for input in self.input_opts.iter() {
            let mut i = 0;
            while i < string.len() {
                if !string.is_match(input, i) {
                    i += 1;
                    continue;
                }
                // input matches

                let mut is_context_match = self.pre_context_opts.len() == 0;
                for pre in self.pre_context_opts.iter() {
                    if i < pre.len() {
                        continue;
                    }
                    if string.is_match(pre, i - pre.len()) {
                        is_context_match = true;
                    }
                }
                if !is_context_match {
                    break;
                }
                // precontext matches

                is_context_match = self.post_context_opts.len() == 0;
                for post in self.post_context_opts.iter() {
                    if string.is_match(post, i + input.len()) {
                        is_context_match = true;
                    }
                }
                if !is_context_match {
                    break;
                }
                // postcontext matches

                // input, precondition, and postcondition all match, so we apply the change
                let from_index = i;
                let to_index = i + input.len();

                // if input and output are the same length, add the segments of corresponding indices
                if self.output.len() == to_index - from_index {
                    for i in from_index..to_index {
                        let new_seg = string[i].clone() + self.output[i - from_index].clone();
                        string[i] = new_seg;
                    }
                } else {
                    // simple splice
                    string.replace(from_index, to_index, &self.output);
                }
                i += self.output.len();
            }
        }
        return Result::Ok(string);
    }
}
