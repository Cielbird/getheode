use crate::phonology::{
    rule::{
        PhonoRule,
        parse::{PhonoRuleParseOpts, RuleElements, parse_rule_patterns},
    },
    string::PhonoString,
};

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
    pub fn parse(input: &str) -> Result<Self, ()> {
        let (_, patterns) =
            parse_rule_patterns(input, PhonoRuleParseOpts::default()).map_err(|_| ())?;
        let elements = RuleElements::from_strings(patterns.enumerate())?;
        Ok(Self {
            rule_text: input.to_string(),
            rules: elements
                .into_iter()
                .map(super::compile::compile_rule)
                .collect(),
        })
    }
}
