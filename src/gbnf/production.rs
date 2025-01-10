use std::rc::Rc;

use crate::{error::{Error, Result}, phoneme::Phoneme};
use super::expression::Expression;

/// struct that prepresents an assignment written as 
/// <lfs> ::= <rhs>
/// the <lhs> must be a non-terminal (see Term struct)
/// the <rhs> is an array of 
pub struct Production {
    pub lhs: String,
    pub rhs: Vec<Expression>
}

impl Production {
    pub fn from_string(production_str: &str, phoneme_inv: &Vec<Rc<Phoneme>>) -> Result<Self> {
        let production_str = production_str.trim();

        // Split line into lhs and rhs
        let parts: Vec<&str> = production_str.split("::=").map(str::trim).collect();
        if parts.len() != 2 {
            return Err(Error::GBNFParsingError(format!("Invalid production: {}", production_str)));
        }

        let mut lhs = parts[0].trim().to_string();
        if !lhs.starts_with('<') || !lhs.ends_with('>') {
            return Err(Error::GBNFParsingError(format!("Invalid non-terminal: {}", lhs)));
        }
        // trim angle brackets from lhs
        lhs = lhs.trim_matches(|c| c == '<' || c == '>').to_string();

        let rhs = parts[1].trim();
        let expressions = Expression::parse_expressions(rhs, phoneme_inv)?;

        return Ok(Production { lhs, rhs: expressions });
    }
}
