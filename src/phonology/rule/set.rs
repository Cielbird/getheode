use crate::phonology::{
    rule::{
        PhonoRule,
        parse::{PhonoRuleParseOpts, RuleElements, parse_rule_patterns},
    },
    string::PhonoString,
};

use super::compile::compile_rule;
use crate::error::*;

pub struct PhonoRuleSet {
    pub rule_text: String,
    pub rules: Vec<PhonoRule>,
}

impl PhonoRuleSet {
    pub fn apply(&self, mut string: PhonoString) -> PhonoString {
        for rule in &self.rules {
            let mut matches = rule.find(string.clone());
            matches.sort_by(|a, b| b.range.start.cmp(&a.range.start));
            for m in matches {
                string = string.replace_range(m.range, m.replace_with).unwrap();
            }
        }
        string
    }

    #[allow(clippy::result_unit_err)] // TODO make error types
    pub fn parse(input: &str, opts: PhonoRuleParseOpts) -> Result<Self> {
        let (rem, patterns) = parse_rule_patterns(input, opts).map_err(Error::other)?;
        let elements = RuleElements::from_strings(patterns.enumerate())?;

        if !rem.is_empty() {
            return Err(Error::other(format!(
                "Couldn't parse rule set \"{input}\", remainder was {rem}"
            )));
        }
        Ok(Self {
            rule_text: input.to_string(),
            rules: elements.into_iter().map(compile_rule).collect(),
        })
    }
}
