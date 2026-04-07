use crate::phonology::rule::PhonoRule;

pub struct PhonoRuleSet {
    rule_text: String,
    rules: Vec<PhonoRule>,
}
impl PhonoRuleSet {
    pub(crate) fn new() -> Self {
        Self {
            rule_text: todo!(),
            rules: todo!(),
        }
    }
}
