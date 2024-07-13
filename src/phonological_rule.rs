use crate::{errors::GetheodeError, segment_string::SegmentString};
use regex::Regex;
use core::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct PhonologicalRule {
    pub input_opts: Vec<SegmentString>,
    pub output: SegmentString,
    pub pre_context_opts: Vec<SegmentString>,
    pub post_context_opts: Vec<SegmentString>
}

impl PhonologicalRule {
    pub fn new(rule_str: &str) -> Result<Self, GetheodeError> {
        let input_str: String;
        let output_str: String;
        let pre_context_str: String;
        let post_context_str: String;
        // parse rule for input, output and contexts
        {
            let p = r"\s*(.+)\s*->\s*([^\/]+)(?:\s*\/\s*([^_]+)\s*_\s*([^_]+))?";
            let re = Regex::new(p).unwrap();
            let mut iter = re.captures_iter(rule_str);
            let first = iter.next();
            if first.is_none() {
                return Err(GetheodeError::PhonologicalRuleParsingError(rule_str.to_string()));
            }
            // there should not be more than one capture
            if iter.next().is_some() {
                return Err(GetheodeError::PhonologicalRuleParsingError(rule_str.to_string()));
            }
            let capts = first.unwrap();
            input_str = capts[1].to_string();
            output_str = capts[2].to_string();
            pre_context_str = capts[3].to_string();
            post_context_str = capts[4].to_string();
        }

        let brackets_re = Regex::new(r"\{\}").unwrap();

        // parse input
        let mut input_opts = Vec::new();
        {
            let has_brackets = brackets_re.is_match(&input_str);
            let opts: Vec<&str>;
            if has_brackets {
                opts = input_str.split(",").collect();
            } else {
                opts = vec![&input_str];
            }
            for opt in opts {
                match SegmentString::from_string(opt.trim()) {
                    Ok(seg) => input_opts.push(seg),
                    Err(e) => return Err(e)
                }
            }
        }

        // parse output
        let output;
        match SegmentString::from_string(&output_str.trim()) {
            Ok(seg_str) => output = seg_str,
            Err(e) => return Err(e)
        }


        // parse pre-context
        let mut precontext_opts = Vec::new();
        {
            let has_brackets = brackets_re.is_match(&pre_context_str);
            let opts: Vec<&str>;
            if has_brackets {
                opts = pre_context_str.split(",").collect();
            } else {
                opts = vec![&pre_context_str];
            }
            for opt in opts {
                match SegmentString::from_string(opt.trim()) {
                    Ok(seg) => precontext_opts.push(seg),
                    Err(e) => return Err(e)
                }
            }
        }

        // parse post-context
        let mut postcontext_opts = Vec::new();
        {
            let has_brackets = brackets_re.is_match(&post_context_str);
            let opts: Vec<&str>;
            if has_brackets {
                opts = post_context_str.split(",").collect();
            } else {
                opts = vec![&post_context_str];
            }
            for opt in opts {
                match SegmentString::from_string(opt.trim()) {
                    Ok(seg) => postcontext_opts.push(seg),
                    Err(e) => return Err(e)
                }
            }
        }
        
        return Ok(PhonologicalRule {
            input_opts: input_opts,
            output: output,
            pre_context_opts: precontext_opts,
            post_context_opts: postcontext_opts
        });
    }

    pub fn apply_rule(self, string: &SegmentString) -> Result<SegmentString, String> {
        // range of indices (inclusive, exclusive) in the string matching the input.
        // ordered smallest to largest
        let mut match_ranges: Vec<(usize,usize)> = vec![];
        // match input
        for input in self.input_opts.iter() {
            let input_len = input.len();
            for string_index in 0..(string.len() - input_len + 1) {
                let mut is_input_match: bool = true;
                for (input_index, input_seg) in input.iter().enumerate() {
                    let seg = &string[string_index + input_index];
                    if !seg.matches(input_seg) {
                        is_input_match = false;
                        break;
                    }
                }
                if !is_input_match {
                    continue
                }
                // check preconditions
                let mut pre_condition_matches = false;
                for pre_contex in self.pre_context_opts.iter() {
                    let pre_context_len = pre_contex.len();
                    // if the precontext is too long to fit in the string, skip
                    if string_index < pre_context_len {
                        continue;
                    }
                    let mut is_match: bool = true;
                    for (pre_context_index, pre_context_seg) in pre_contex.iter().enumerate() {
                        let i = string_index - pre_context_len + pre_context_index;
                        let seg = &string[i];
                        if !seg.matches(pre_context_seg) {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        pre_condition_matches = true;
                        break;
                    }
                }
                if !pre_condition_matches {
                    continue;
                }

                // check postconditions
                let mut post_condition_matches = false;
                for post_contex in self.post_context_opts.iter() {
                    let post_context_len = post_contex.len();
                    // if post-context goes beyond the string's segment length
                    if string_index + input_len + post_context_len > string.len() {
                        continue;
                    }
                    let mut is_match: bool = true;
                    for (post_context_index, post_context_seg) in post_contex.iter().enumerate() {
                        let i = string_index + input_len + post_context_index;
                        let seg = &string[i];
                        if !seg.matches(post_context_seg) {
                            is_match = false;
                            break;
                        }
                    }
                    if is_match {
                        post_condition_matches = true;
                        break;
                    }
                }
                if !post_condition_matches {
                    continue;
                }
                // input, precondition, and postcondition all match, so we add it to the list
                match_ranges.push((string_index, string_index + input_len));
            }
        }
        // if no matches found, error
        if match_ranges.len() == 0 {
            return Result::Err("No matches found".to_string());
        }
        // replace input with output
        // only take first match
        let mut new_segments = SegmentString::new();
        let from_index = match_ranges[0].0;
        let to_index = match_ranges[0].1;
        // push segments before match
        for i in 0..from_index {
            new_segments.push(string[i].clone());
        }
        // if input and output are the same length, add the segments of corresponding indices
        if self.output.len() == to_index - from_index {
            for i in from_index..to_index {
                let new_seg = string[i].clone() + self.output[i-from_index].clone();
                new_segments.push(new_seg);
            }
        } else {
            // simple splice
            for i in from_index..to_index {
                new_segments.push(self.output[i-from_index].clone());
            }
        }
        // push segments after match
        for i in to_index..string.len() {
            new_segments.push(string[i].clone());
        }
        return Result::Ok(new_segments);
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for PhonologicalRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // format input
        let mut input_str = String::new();
        if self.input_opts.len() == 1 {
            input_str = format!("{}", self.input_opts[0]);
        } else {
            input_str.push_str("{");
            for opt in &self.input_opts {
                input_str.push_str(&format!("{}, ", opt));
            }
            input_str.push_str("}");
        }

        // format output
        let output_str = format!("{}", self.output);

        // format rule, do we have context or not?
        if self.pre_context_opts.len() == 0 && self.post_context_opts.len() == 0 {
            return write!(f, "{} -> {}", input_str, output_str);
        } else {
            let mut context_str = format!("_");
            for pre in &self.pre_context_opts {
                context_str = format!("{}{}", pre, context_str);
            }
            for post in &self.post_context_opts {
                context_str = format!("{}{}", context_str, post);
            }
            return write!(f, "{} -> {} / {}", input_str, output_str, context_str);
        }
    }
}
