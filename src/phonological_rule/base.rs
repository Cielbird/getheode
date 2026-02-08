use crate::{error::Result, segment::{PhonologicalElement::*, PhonologicalString, PhonologicalStringPattern}};

#[derive(Debug)]
pub struct PhonologicalRule {
    pub input_opts: Vec<PhonologicalStringPattern>,
    pub output: PhonologicalString,
    /// What must procede the match
    pub pre_context_opts: Vec<PhonologicalStringPattern>,
    /// What must follow the match
    pub post_context_opts: Vec<PhonologicalStringPattern>,
    pub(crate) strategy: PhonologicalRuleStrategy
}

#[derive(Debug)]
pub(crate) enum PhonologicalRuleStrategy {
    Replace, // if the output replaces the matched string
    Add, // if the output is `added` to the matched string
}

impl PhonologicalRule {
    pub fn apply(&self, s: &PhonologicalString) -> Result<PhonologicalString> {
        // TODO skip all boundaries
        // string we will be modifying and returning
        let mut string = s.clone();

        for input in self.input_opts.iter() {
            let mut i = 0;
            while i < string.elements.len() {
                if !input.is_match(&string, i) {
                    i += 1;
                    continue;
                }
                // input matches

                let mut is_context_match = self.pre_context_opts.is_empty();
                for pre in self.pre_context_opts.iter() {
                    if pre.is_match(&string, i - pre.element_len()) {
                        is_context_match = true;
                    }
                }
                if !is_context_match {
                    break;
                }
                // precontext matches

                is_context_match = self.post_context_opts.is_empty();
                for post in self.post_context_opts.iter() {
                    if post.is_match(&string, i + input.element_len()) {
                        is_context_match = true;
                    }
                }
                if !is_context_match {
                    break;
                }
                // postcontext matches now

                // input, precondition, and postcondition all match, so we apply the change
                let from_index = i;
                let to_index = i + input.element_len();

                // if input and output are the same length, add the segments of corresponding indices
                match self.strategy {
                    PhonologicalRuleStrategy::Replace => {
                        // simple splice
                        string.replace(from_index, to_index, self.output.clone());
                    },
                    PhonologicalRuleStrategy::Add => { 
                        for i in from_index..to_index {
                            let preexisting = string.elements[i].clone();
                            let rule_output = self.output.elements[i - from_index].clone();
                            
                            if let SegmentElement(preexisting) = preexisting {
                                if let SegmentElement(rule_output) = rule_output {
                                    let new_seg = preexisting + rule_output;
                                    string.elements[i] = SegmentElement(new_seg);
                                } else {
                                    panic!("This is an uncovered case ! Starts with writing a better match() function for PhonologicalStringPattern")
                                }
                            }
                        }
                    },
                }
                i += self.output.element_len();
            }
        }
        Result::Ok(string)
    }
}
