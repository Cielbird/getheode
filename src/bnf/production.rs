use super::expression::Expression;

/// struct that prepresents an assignment written as 
/// <lfs> ::= <rhs>
/// the <lhs> must be a non-terminal (see Term struct)
/// the <rhs> is an array of 
pub struct Production {
    lhs: String,
    rhs: Vec<Expression>
}
