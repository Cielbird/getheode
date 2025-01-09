use regex::Regex;

use crate::segment_string::SegmentString;
use crate::error::{Result, Error};

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

impl Term {
    /// Parses a sequence of terms in a gbnf format
    /// Can be seperated by whitespace. each term is terminal <asdf> or nonterminal [asdf].
    /// Example:
    ///     <vowel> [fa]<C>
    /// Returns a result of a vector of terms.
    pub fn parse_terms(alt: &str) -> Result<Vec<Term>> {
        let mut terms = Vec::new();
        let pattern = r"^(<[^<>]*>|\[[^\[\]]*\])";
        let regex = Regex::new(pattern).expect("Invalid regex");

        let mut remaining_input = alt;

        while let Some(mat) = regex.find(remaining_input) {
            let matched_text = mat.as_str().trim();
            if matched_text.starts_with('[') {
                // Terminal
                let content = &matched_text.trim_matches(|c| c == '[' || c == ']').to_string();
                terms.push(Term::Terminal(SegmentString::new(&content)?));

            } else if matched_text.starts_with('<') {
                // Non-Terminal
                let content = &matched_text.trim_matches(|c| c == '<' || c == '>').to_string();

                terms.push(Term::NonTerminal(content.to_string()));
            }

            // Advance input beyond the current match
            remaining_input = &remaining_input[mat.end()..].trim_start();
        }

        if remaining_input.is_empty() {
            return Ok(terms);
        }
        return Err(Error::GBNFParsingError(format!("Invalid terms: {}", alt)));
    }
}
