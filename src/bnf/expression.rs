use super::term::Term;

/// An Expression is sequence of any number of Terms
pub struct Expression {
    terms: Vec<Term>
}