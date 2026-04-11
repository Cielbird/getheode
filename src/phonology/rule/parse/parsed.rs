use std::iter::zip;

use crate::phonology::rule::{PhonoRule, parse::{pattern::Pattern, elem::{Element}}};

pub struct PhonoRuleSet {
    pub(crate) text: String,
    pub(crate) rules: Vec<PhonoRule>,
}

/// rule, with branching parsed
pub struct RulePatternsParsed<'a> {
    pub(crate) input: Vec<Pattern<'a>>,
    pub(crate) output: Vec<Vec<&'a str>>, // no branching in the output : deterministic
    pub(crate) pre_context: Option<Pattern<'a>>,
    pub(crate) post_context: Option<Pattern<'a>>,
}



/// Represents a written rule, void of all branches
pub struct StrictParsedRule {
    input: Vec<Element>,
    output: Vec<Element>,
    pre_ctx: Vec<Element>,
    post_ctx: Vec<Element>,
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
