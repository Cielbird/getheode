use std::rc::Rc;

use crate::{error::{Error, Result}, phoneme::Phoneme};

use super::term::Term;

/// An Expression is sequence of any number of Terms
pub struct Expression {
    pub terms: Vec<Term>
}

impl Expression {
    /// Parses a list gbnf expressions seperated by "|"
    /// Example:
    ///     <A>| <B> | a | ez<A>
    /// if the expression is 
    ///     <x> | []
    /// the second item will be a empty terminal term (empty segment string).
    pub fn parse_expressions(rhs: &str, phoneme_inv: &Vec<Rc<Phoneme>>) -> Result<Vec<Expression>> {
        let mut expressions = Vec::new();

        // Split alternatives (|)
        for alt in rhs.split('|').map(str::trim) {
            let terms;
            if alt == "[]" {
                terms = vec![Term::None];
            } else {
                terms = Term::parse_terms(alt, phoneme_inv)?;
            }
            expressions.push(Expression { terms });
        }

        Ok(expressions)
    }
}
