use crate::error::*;
use crate::phoneme::{PhonemeBank, PhonemeId};

/// Parses the next phoneme from bank of phonemes in a string, returns the id of the next phoneme 
/// and the string
pub fn parse_next_phoneme<'a>(phonemes: &PhonemeBank, term_str: &'a str) -> Option<(PhonemeId, &'a str)> {
    for (id, phoneme) in &phonemes.phonemes {
        let sym = &phoneme.symbol;
        if term_str.starts_with(sym) {
            // first match
            let term_str = &term_str[sym.len()..];
            return Some((*id, term_str));
        }
    }

    None
}

/// parses a string and identifies the sequence of phonemes used
/// the phoneme symbols are used to identify them.
/// first match found is used, so if there are issues with findinf phonemes,
/// consider reordering the phoneme inventory.
pub fn parse_phonemes(phonemes: &PhonemeBank, input: &str) -> Result<Vec<PhonemeId>> {
    // remove all whitespace
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    let mut remaining_input: &str = &input;
    let mut result: Vec<PhonemeId> = vec![];

    while !remaining_input.is_empty() {
        // the phoneme symbol that matches is chosen
        let mut found = false;
        for (id, phoneme) in &phonemes.phonemes {
            let sym = &phoneme.symbol;
            if remaining_input.starts_with(sym) {
                // first match
                result.push(*id);
                found = true;
                remaining_input = &remaining_input[sym.len()..];
                break;
            }
        }
        if !found {
            return Err(Error::PhonemeSymbolParsingError(format!(
                "Could not parse the phonemes of the string \"{}\"",
                input
            )));
        }
    }
    return Ok(result);
}
