use std::rc::Rc;

use regex::Regex;

use crate::lect::Lect;
use crate::phoneme::Phoneme;
use crate::error::{Result, Error};

/// a Term can represent a Terminal or NonTerminal node
/// a Terminal node is a segment string used in the syntax.
/// a NonTerminal node is used to represent an intermediate symbol, used as lhs of a production
#[derive(Clone, Debug)] //Deserialize, Serialize, 
pub enum Term {
    /// A term which cannot be expanded further via productions
    Terminal(Rc<Phoneme>),
    /// A term which may be be expanded further via productions
    NonTerminal(String),
    None
}

impl Term {
    /// Parses a sequence of terms in a gbnf format
    /// Whitespace is entirely ignored. each term is terminal (surrounded with angle brackets) or
    ///  nonterminal (the symbol of a phoneme).
    /// Example:
    ///     "<vowel> fa<C>""
    /// Returns a result of a vector of terms
    pub fn parse_terms(input: &str, phoneme_inv: &Vec<Rc<Phoneme>>) -> Result<Vec<Term>> {
        // remove all whitespace
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        let mut terms = Vec::new();
        // matches either a group of phonemes or a non-terminal production, 
        // at the front of the remaining string
        let pattern = r"^(<[^<>]+>|[^<>]+)";
        let regex = Regex::new(pattern).expect("Invalid regex");

        let mut remaining_input: &str = &input;

        while let Some(mat) = regex.find(remaining_input) {
            let matched_text = mat.as_str().trim();
            if matched_text.starts_with('<') {
                // Non-Terminal
                let content = matched_text.trim_matches(|c| c == '<' || c == '>').to_string();

                terms.push(Term::NonTerminal(content));
            } else {
                // Terminal (one or more)
                let content = &matched_text.to_string();
                // parse the phonemes' symbols and add their references
                for x in Phoneme::parse_phonemes(content, phoneme_inv)? {
                    terms.push(Term::Terminal(x));
                }            
            } 

            // Advance input beyond the current match
            remaining_input = &remaining_input[mat.end()..];
        }

        if remaining_input.is_empty() {
            return Ok(terms);
        }
        return Err(Error::GBNFParsingError(format!("Invalid terms: {}", input)));
    }
}
