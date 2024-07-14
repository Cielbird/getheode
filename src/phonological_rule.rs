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
            let p = r"\s*(.+)\s*->\s*([^\/]+)\s*(?:\/\s*([^_]*)\s*_\s*([^_]*))?";
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
            input_str = capts[1].trim().to_string();
            output_str = capts[2].trim().to_string();
            pre_context_str = capts[3].trim().to_string();
            post_context_str = capts[4].trim().to_string();
        }

        // parse input
        let input_opts;
        match parse_seg_string_opts(&input_str) {
            Ok(seg_str_opts) => input_opts = seg_str_opts,
            Err(e) => return Err(e)
        }

        // parse output
        let output;
        match SegmentString::from_string(&output_str.trim()) {
            Ok(seg_str) => output = seg_str,
            Err(e) => return Err(e)
        }

        // parse pre-context
        let mut pre_context_opts = Vec::new();
        if pre_context_str != ""{
            match parse_seg_string_opts(&pre_context_str) {
                Ok(seg_str_opts) => pre_context_opts = seg_str_opts,
                Err(e) => return Err(e)
            }
        }

        // parse post-context
        let mut post_context_opts = Vec::new();
        if post_context_str != ""{
            match parse_seg_string_opts(&post_context_str) {
                Ok(seg_str_opts) => post_context_opts = seg_str_opts,
                Err(e) => return Err(e)
            }
        }

        return Ok(PhonologicalRule {
            input_opts: input_opts,
            output: output,
            pre_context_opts: pre_context_opts,
            post_context_opts: post_context_opts
        });
    }

    pub fn apply_rule(self, s: &SegmentString) -> Result<SegmentString, String> {
        // string we will be modifying and returning
        let mut string = s.clone();

        // match pattern and apply change before matching next
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
                let mut pre_condition_matches = self.pre_context_opts.len() == 0;
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
                let mut post_condition_matches = self.post_context_opts.len() == 0;
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

                // input, precondition, and postcondition all match, so we apply the change
                let from_index = string_index;
                let to_index = string_index+input_len;

                // if input and output are the same length, add the segments of corresponding indices
                if self.output.len() == to_index - from_index {
                    for i in from_index..to_index {
                        let new_seg = string[i].clone() + self.output[i-from_index].clone();
                        string[i] = new_seg;
                    }
                } else {
                    // simple splice
                    string.drain(from_index..to_index);
                    for i in 0..self.output.len() {
                        string.insert(from_index + i, self.output[i].clone());
                    }
                }
            }
        }
        return Result::Ok(string);
    }
}

/// returns the segment's defined non-NA features, concatenated
impl Display for PhonologicalRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // format input
        let input_str = format_seg_string_opts(&self.input_opts);

        // format output
        let output_str = format!("{}", self.output);

        // format rule, do we have context or not?
        if self.pre_context_opts.len() == 0 && self.post_context_opts.len() == 0 {
            return write!(f, "{} -> {}", input_str, output_str);
        } else {
            // format precontext
            let pre_context_str = format_seg_string_opts(&self.pre_context_opts);

            // format postcontext
            let post_context_str = format_seg_string_opts(&self.post_context_opts);

            return write!(f, "{} -> {} / {}_{}", input_str, output_str, pre_context_str, post_context_str);
        }
    }
}

/// parses a list of segment strings in brackets: {x, y, z...}.
/// allows for a single segmentstring, in which case returns a size 1 vector
/// extra commas anywhere are allowed: {a, b, c,}
fn parse_seg_string_opts(s: &str) -> Result<Vec<SegmentString>, GetheodeError> {
    let s = s.trim();
    let mut seg_str_opts = Vec::new();
    let has_brackets = s.starts_with('{') && s.ends_with('}');
    let opts: Vec<&str>;
    if has_brackets {
        let inner = &s[1..(s.len() - 1)];
        opts = inner.split(",").collect();
    } else {
        opts = vec![&s];
    }
    for opt in opts {
        let opt = opt.trim();
        if opt == ""{
            continue;
        }
        match SegmentString::from_string(opt) {
            Ok(seg) => seg_str_opts.push(seg),
            Err(e) => return Err(e)
        }
    }
    return Ok(seg_str_opts);
}

/// returns a formated string of a vector of segment strings in brackets: {x, y, z...}.
fn format_seg_string_opts(opts: &Vec<SegmentString>) -> String {
    let mut str = String::new();
    if opts.len() == 0 {
        return "".to_owned();
    } else if opts.len() == 1 {
        str = format!("{}", opts[0]);
    } else {
        str.push_str("{");
        for opt in opts {
            str.push_str(&format!("{},", opt));
        }
        str.push_str("}");
    }
    return str;
}