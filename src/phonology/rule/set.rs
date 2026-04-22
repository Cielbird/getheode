use crate::phonology::rule::{
    PhonoRule,
    parse::{PhonoRuleParseOpts, RuleElements, parse_rule_patterns},
};

pub struct PhonoRuleSet {
    pub rule_text: String,
    pub rules: Vec<PhonoRule>,
}

impl PhonoRuleSet {
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
