use crate::phonology::rule::PhonoRule;

pub struct PhonoRuleSet {
    pub rule_text: String,
    pub rules: Vec<PhonoRule>,
}

impl PhonoRuleSet {
    pub(crate) fn new(rule_text: String, rules: Vec<PhonoRule>) -> Self {
        Self { rule_text, rules }
    }
}
