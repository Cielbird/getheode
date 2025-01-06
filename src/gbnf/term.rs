use crate::segment_string::SegmentString;

/// a Term can represent a Terminal or NonTerminal node
/// a Terminal node is a segment string used in the syntax.
/// a NonTerminal node is used to represent an intermediate symbol, used as lhs of a production
#[derive(Clone, Debug)] //Deserialize, Serialize, 
pub enum Term {
    /// A term which cannot be expanded further via productions
    Terminal(SegmentString),
    /// A term which may be be expanded further via productions
    NonTerminal(String),
}
