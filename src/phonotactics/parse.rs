//! Parsing the getheode phonotactics format

use crate::{
    error::*,
    phoneme::{PhonemeBank, parse_next_phoneme},
    phonotactics::{Phonotactics, Term},
};

pub trait FromGbnf {
    fn from_gbnf(phonemes: &PhonemeBank, contents: String) -> Result<Phonotactics>;
}

impl FromGbnf for Phonotactics {
    /// Read contents in gbnf format into
    fn from_gbnf(phonemes: &PhonemeBank, contents: String) -> Result<Self> {
        let mut phonotactics = Self::new();
        let lines = contents.split("\n");
        for line in lines {
            let line = line.trim_start();
            if line.starts_with("#") || line.is_empty() {
                continue;
            }
            let (lhs, rhs) = parse_gbnf_production(phonemes, line)?;
            phonotactics.add_production(lhs, rhs)
        }

        Ok(phonotactics)
    }
}

/// Parse a production in GBNF format.
///
/// Return
pub(crate) fn parse_gbnf_production(
    phonemes: &PhonemeBank,
    prod: &str,
) -> Result<(String, Vec<Vec<Term>>)> {
    // Split line into lhs and rhs
    let parts: Vec<&str> = prod.split("::=").map(str::trim).collect();
    if parts.len() != 2 {
        return Err(Error::GBNFParsingError(format!(
            "Invalid production: {}",
            prod
        )));
    }

    let lhs_str = parts[0].trim();
    if !lhs_str.starts_with('<') || !lhs_str.ends_with('>') {
        return Err(Error::GBNFParsingError(format!(
            "Invalid lhs term: {}",
            lhs_str
        )));
    }

    let lhs = get_gbnf_term_symbols(phonemes, lhs_str)
        .ok_or(Error::Other(format!("Invalid production Lhs: {lhs_str:?}")))?;
    let lhs = match lhs.as_slice() {
        [Term::NonTerminal(lhs)] => lhs.to_owned(),
        _ => {
            return Err(Error::Other(format!("Invalid production Lhs: {lhs:?}")));
        }
    };

    let rhs_str = parts[1].trim();
    let rhs_terms = parse_gbnf_production_rhs(phonemes, rhs_str)
        .ok_or(Error::Other(format!("Invalid production Rhs: {lhs_str:?}")))?;

    Ok((lhs, rhs_terms))
}

/// Parse the terms of the rhs of a pronotactic production in GBNF format.
///
/// Sequences of terms is an alternative, seperated by pipes. Example: "<one>aeiou | <two><three>"
///
/// * `phonemes` - the phonemes to use when parsing terms
/// * `rhs_str` - the string to parse
///
/// # Returns
///
/// None if parsing failed. Otherwise, a vec of each alternative, which is a vec of terms.
fn parse_gbnf_production_rhs(phonemes: &PhonemeBank, rhs_str: &str) -> Option<Vec<Vec<Term>>> {
    let mut rhs_symbols = vec![];

    // Split alternatives (|)
    for alt in rhs_str.split('|').map(str::trim) {
        if alt == "[]" {
            rhs_symbols.push(vec![]);
        } else {
            let terms = get_gbnf_term_symbols(phonemes, alt)?;
            rhs_symbols.push(terms);
        }
    }

    Some(rhs_symbols)
}

/// Get a sequence of term symbols in GBNF format.
/// Example: "<vowel><cons>e" is a sequence of three terms
fn get_gbnf_term_symbols(phonemes: &PhonemeBank, mut term_str: &str) -> Option<Vec<Term>> {
    let mut terms = vec![];
    while !term_str.is_empty() {
        let (term, remaining) = get_gnbf_next_term(phonemes, term_str)?;
        term_str = remaining;
        terms.push(term);
    }

    Some(terms)
}

/// Parse the next term in a string in GBNF format.
///
/// The string is made up of phoneme symbols and non-terminal terms (example: <term>)
///
/// # Parameters
///
/// * `term_str` - Input for parsing
///
/// # Returns
///
/// A tuple with the parsed symbol and the remaining string.
fn get_gnbf_next_term<'a>(
    phonemes: &PhonemeBank,
    mut term_str: &'a str,
) -> Option<(Term, &'a str)> {
    // trim whitespace
    term_str = term_str.trim();

    if term_str.starts_with('<') {
        term_str = &term_str[1..];
        let (content, remaining) = term_str.split_once('>')?;

        Some((Term::NonTerminal(content.to_owned()), remaining))
    } else {
        let (id, remaining) = parse_next_phoneme(phonemes, term_str)?;

        Some((Term::Terminal(id), remaining))
    }
}
