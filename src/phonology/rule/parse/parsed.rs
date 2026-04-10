use crate::phonology::rule::{PhonoRule, parse::tree::{ParsedRuleElem, ParsedRulePattern}};

pub struct PhonoRuleSet {
    pub(crate) text: String,
    pub(crate) rules: Vec<PhonoRule>,
}

/// Represents a written rule, in element sequence form, with branching
pub struct ParsedRule {
    pub(crate) input: Vec<ParsedRulePattern>,
    pub(crate) output: Vec<Vec<ParsedRuleElem>>, // no branching in the output
    pub(crate) pre_context: Option<ParsedRulePattern>,
    pub(crate) post_context: Option<ParsedRulePattern>,
}


/// Represents a written rule, void of all branches
pub struct StrictParsedRule {
    input: Vec<ParsedRuleElem>,
    output: Vec<ParsedRuleElem>,
    pre_ctx: Vec<ParsedRuleElem>,
    post_ctx: Vec<ParsedRuleElem>,
}


impl StrictParsedRule {
    /// Generate tags for the non-defined, untagged segments of this rule
    /// Also tag the segments of contexts
    pub(crate) fn complete_tags(&self) {
        
    }
    
    pub(crate) fn synthesize(&self) -> Vec<PhonoRule> {
        todo!()
    }
}
