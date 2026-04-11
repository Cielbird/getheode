use crate::phonology::rule::parse::pattern::Pattern;

/// rule, with branching parsed
pub struct RulePatternsParsed<'a> {
    pub(crate) input: Vec<Pattern<'a>>,
    pub(crate) output: Vec<Vec<&'a str>>, // no branching in the output : deterministic
    pub(crate) _pre_context: Option<Pattern<'a>>,
    pub(crate) _post_context: Option<Pattern<'a>>,
}
