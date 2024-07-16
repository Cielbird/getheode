use crate::{errors::GetheodeError, segment_string::SegmentString};
use core::fmt;
use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
pub struct PhonologicalRule {
    pub input_opts: Vec<SegmentString>,
    pub output: SegmentString,
    pub pre_context_opts: Vec<SegmentString>,
    pub post_context_opts: Vec<SegmentString>,
}

impl PhonologicalRule {
    pub fn new(rule_str: &str) -> Result<Self, GetheodeError> {
        let input_str: &str;
        let output_str: &str;
        let pre_context_str: &str;
        let post_context_str: &str;
        // parse rule for input, output and contexts
        let p = r"\s*(.+)\s*->\s*([^\/]+)\s*(?:\/\s*([^_]*)\s*_\s*([^_]*))?";
        let re = Regex::new(p).unwrap();
        let mut iter = re.captures_iter(rule_str);
        let first = iter.next();
        if first.is_none() {
            return Err(GetheodeError::PhonologicalRuleParsingError(
                rule_str.to_string(),
            ));
        }
        // there should not be more than one capture
        if iter.next().is_some() {
            return Err(GetheodeError::PhonologicalRuleParsingError(
                rule_str.to_string(),
            ));
        }
        let capts = first.unwrap();
        input_str = capts[1].trim();
        output_str = capts[2].trim();
        // if there is context
        if let Some(x) = capts.get(3) {
            pre_context_str = capts[3].trim();
            post_context_str = capts[4].trim();
        } else {
            pre_context_str = "";
            post_context_str = "";
        }

        // parse input
        let mut input_opts;
        match parse_seg_string_opts(&input_str) {
            Ok(seg_str_opts) => input_opts = seg_str_opts,
            Err(e) => return Err(e),
        }
        // input should never have no options
        if input_opts.len() == 0 {
            input_opts.push(SegmentString::new("").unwrap());
        }

        // parse output
        let output;
        match SegmentString::new(&output_str) {
            Ok(seg_str) => output = seg_str,
            Err(e) => return Err(e),
        }

        // parse pre-context
        let mut pre_context_opts = Vec::new();
        if pre_context_str != "" {
            match parse_seg_string_opts(&pre_context_str) {
                Ok(seg_str_opts) => pre_context_opts = seg_str_opts,
                Err(e) => return Err(e),
            }
        }

        // parse post-context
        let mut post_context_opts = Vec::new();
        if post_context_str != "" {
            match parse_seg_string_opts(&post_context_str) {
                Ok(seg_str_opts) => post_context_opts = seg_str_opts,
                Err(e) => return Err(e),
            }
        }

        return Ok(PhonologicalRule {
            input_opts: input_opts,
            output: output,
            pre_context_opts: pre_context_opts,
            post_context_opts: post_context_opts,
        });
    }

    pub fn apply(&self, s: &SegmentString) -> Result<SegmentString, String> {
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

            return write!(
                f,
                "{} -> {} / {}_{}",
                input_str, output_str, pre_context_str, post_context_str
            );
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
        if opt == "" {
            continue;
        }
        match SegmentString::new(opt) {
            Ok(seg) => seg_str_opts.push(seg.clone()),
            Err(e) => return Err(e),
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
