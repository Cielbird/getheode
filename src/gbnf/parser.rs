use regex::Regex;

use crate::segment_string::SegmentString;

use super::{expression::Expression, grammar::Grammar, production::Production, term::Term};

pub fn parse_bnf(input: &str) -> Result<Grammar, String> {
    let mut productions = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        // Skip empty lines or comments
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        // Split line into lhs and rhs
        let parts: Vec<&str> = line.split("::=").map(str::trim).collect();
        if parts.len() != 2 {
            return Err(format!("Invalid production: {}", line));
        }

        let lhs = parts[0].trim().to_string();
        if !lhs.starts_with('<') || !lhs.ends_with('>') {
            return Err(format!("Invalid non-terminal: {}", lhs));
        }

        let rhs = parts[1].trim();
        let expressions = parse_expressions(rhs)?;

        productions.push(Production { lhs, rhs: expressions });
    }

    Ok(Grammar { productions })
}

fn parse_expressions(rhs: &str) -> Result<Vec<Expression>, String> {
    let mut expressions = Vec::new();

    // Split alternatives (|)
    for alt in rhs.split('|').map(str::trim) {
        let terms = parse_terms(alt)?;
        expressions.push(Expression { terms });
    }

    Ok(expressions)
}

fn parse_terms(alt: &str) -> Result<Vec<Term>, String> {
    let mut terms = Vec::new();
    let pattern = r"^(<[^<>]*>|\[[^\[\]]*\])";
    let regex = Regex::new(pattern).expect("Invalid regex");

    let mut remaining_input = alt;

    while let Some(mat) = regex.find(remaining_input) {
        let matched_text = mat.as_str().trim();
        if matched_text.starts_with('[') {
            // Terminal
            let content = &matched_text.trim_matches(|c| c == '[' || c == ']').to_string();
            match SegmentString::new(&content) {
                Ok(segment_str) => {
                    terms.push(Term::Terminal(segment_str));
                },
                Err(_e) => return Err(format!("Invalid terminal term segment string: {}", alt))
            }
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
    return Err(format!("Invalid terms: {}", alt));
}