use crate::error::*;
use crate::phoneme::{PhonemeBank, PhonemeId};
use crate::segment::{FormatSegment, Segment};

/// Defines how to format a phoneme bank as a string
pub trait FormatPhonemeBank {
    fn parse(input: &str) -> Result<Self>
    where
        Self: Sized;
    fn format(&self) -> String;
}

impl FormatPhonemeBank for PhonemeBank {
    fn parse(input: &str) -> Result<Self> {
        let mut result = Self::new();
        for input in input.split('\n') {
            let parts = input.split(':').collect::<Vec<&str>>();
            if parts.len() == 1 {
                // <segment>
                let segment = parts[0].trim();
                result.add(Segment::parse_segment(segment)?, segment.to_string());
            } else {
                // <symbol>: <segment>
                let symbol = parts[0].trim();
                let segment = parts[1].trim();
                result.add(Segment::parse_segment(segment)?, symbol.to_string());
            }
        }

        Ok(result)
    }

    fn format(&self) -> String {
        let mut result = String::new();
        for (_, phoneme) in &self.phonemes {
            let line = format!("{}: {}\n", phoneme.symbol, phoneme.segment);
            result += &line;
        }

        result
    }
}

/// Parses the next phoneme from bank of phonemes in a string, returns the id of the next phoneme
/// and the string
pub fn parse_next_phoneme<'a>(
    phonemes: &PhonemeBank,
    term_str: &'a str,
) -> Option<(PhonemeId, &'a str)> {
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
