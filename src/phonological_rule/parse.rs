use core::fmt;
use std::fmt::Display;

use regex::Regex;

use crate::error::*;
use crate::phonological_rule::PhonologicalRule;
use crate::segment::{FormatSegmentString, SegmentString};

pub trait FormatRuleStr {
    /// Build a phonological rule from a string. For formatting see the README.
    fn parse(rule_str: &str) -> Result<Self>
    where
        Self: Sized;
    /// Build formatted phonological rule string. For formatting see the README.
    fn format(&self) -> String;
}

/// Defines how a phonological realization rules file is is parsed and formatted
pub trait FormatRulesFile: FormatRuleStr {
    fn parse_file(rules: &str) -> Result<Vec<Self>>
    where
        Self: Sized,
    {
        let mut out = vec![];
        for rule in rules.split('\n') {
            let rule = rule.trim();
            // Skip comments with # and empty lines
            if rule.starts_with('#') || rule.is_empty() {
                continue;
            }
            out.push(FormatRuleStr::parse(rule)?);
        }
        Ok(out)
    }
    fn format_file(rules: Vec<PhonologicalRule>) -> String {
        let mut out = String::new();
        for rule in rules {
            out += &FormatRuleStr::format(&rule);
        }
        out
    }
}

impl<T> FormatRulesFile for T where T: FormatRuleStr {}

impl FormatRuleStr for PhonologicalRule {
    fn parse(rule_str: &str) -> Result<PhonologicalRule> {
        let pre_context_str: &str;
        let post_context_str: &str;
        // parse rule for input, output and contexts
        let p = r"\s*(.+)\s*->\s*([^\/]+)\s*(?:\/\s*([^_]*)\s*_\s*([^_]*))?";
        let re = Regex::new(p).unwrap();
        let mut iter = re.captures_iter(rule_str);
        let first = iter.next();
        if first.is_none() {
            return Err(Error::PhonologicalRuleParsingError(rule_str.to_string()));
        }
        // there should not be more than one capture
        if iter.next().is_some() {
            return Err(Error::PhonologicalRuleParsingError(rule_str.to_string()));
        }
        let capts = first.unwrap();
        let input_str: &str = capts[1].trim();
        let output_str: &str = capts[2].trim();
        // if there is context
        if let Some(_x) = capts.get(3) {
            pre_context_str = capts[3].trim();
            post_context_str = capts[4].trim();
        } else {
            pre_context_str = "";
            post_context_str = "";
        }

        // parse input
        let mut input_opts;
        match parse_seg_string_opts(input_str) {
            Ok(seg_str_opts) => input_opts = seg_str_opts,
            Err(e) => return Err(e),
        }
        // input should never have no options
        if input_opts.is_empty() {
            input_opts.push(SegmentString::parse("").unwrap());
        }

        // parse output

        let output = SegmentString::parse(output_str)?;

        // parse pre-context
        let mut pre_context_opts = Vec::new();
        if !pre_context_str.is_empty() {
            match parse_seg_string_opts(pre_context_str) {
                Ok(seg_str_opts) => pre_context_opts = seg_str_opts,
                Err(e) => return Err(e),
            }
        }

        // parse post-context
        let mut post_context_opts = Vec::new();
        if !post_context_str.is_empty() {
            match parse_seg_string_opts(post_context_str) {
                Ok(seg_str_opts) => post_context_opts = seg_str_opts,
                Err(e) => return Err(e),
            }
        }

        Ok(PhonologicalRule {
            input_opts,
            output,
            pre_context_opts,
            post_context_opts,
        })
    }

    fn format(&self) -> String {
        // format input
        let input_str = format_seg_string_opts(&self.input_opts);

        // format output
        let output_str = format!("{}", self.output);

        // format rule, do we have context or not?
        if self.pre_context_opts.is_empty() && self.post_context_opts.is_empty() {
            format!("{} -> {}", input_str, output_str)
        } else {
            // format precontext
            let pre_context_str = format_seg_string_opts(&self.pre_context_opts);

            // format postcontext
            let post_context_str = format_seg_string_opts(&self.post_context_opts);

            format!(
                "{} -> {} / {}_{}",
                input_str, output_str, pre_context_str, post_context_str
            )
        }
    }
}

impl Display for PhonologicalRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// parses a list of segment strings in brackets: {x, y, z...}.
/// allows for a single segmentstring, in which case returns a size 1 vector
/// extra commas anywhere are allowed: {a, b, c,}
fn parse_seg_string_opts(s: &str) -> Result<Vec<SegmentString>> {
    let s = s.trim();
    let mut seg_str_opts = Vec::new();
    let has_brackets = s.starts_with('{') && s.ends_with('}');
    let opts: Vec<&str> = if has_brackets {
        let inner = &s[1..(s.len() - 1)];
        inner.split(",").collect()
    } else {
        vec![&s]
    };
    for opt in opts {
        let opt = opt.trim();
        if opt.is_empty() {
            continue;
        }
        match SegmentString::parse(opt) {
            Ok(seg) => seg_str_opts.push(seg.clone()),
            Err(e) => return Err(e),
        }
    }
    Ok(seg_str_opts)
}

/// returns a formated string of a vector of segment strings in brackets: {x, y, z...}.
fn format_seg_string_opts(opts: &Vec<SegmentString>) -> String {
    let mut str = String::new();
    if opts.is_empty() {
        return "".to_owned();
    } else if opts.len() == 1 {
        str = format!("{}", opts[0]);
    } else {
        str.push('{');
        for opt in opts {
            str.push_str(&format!("{},", opt));
        }
        str.push('}');
    }
    str
}
