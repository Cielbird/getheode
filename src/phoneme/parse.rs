use crate::error::*;
use crate::phoneme::{PhonemeBank, PhonemeString, PhonemeStringSylable};
use crate::segment::Segment;

impl PhonemeBank {
    pub fn parse(input: &str) -> Result<Self> {
        let mut result = Self::new();
        for input in input.split('\n') {
            let input = input.trim();
            // Skip comments with # and empty lines
            if input.starts_with('#') || input.is_empty() {
                continue;
            }
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

    pub fn format(&self) -> String {
        let mut result = String::new();
        for phoneme in self.phonemes.values() {
            let line = format!("{}: {}\n", phoneme.symbol, phoneme.segment);
            result += &line;
        }

        result
    }
}

impl PhonemeString {
    /// parses a string and identifies the sequence of phonemes used
    /// the phoneme symbols are used to identify them.
    /// first match found is used, so if there are issues with finding phonemes,
    /// consider reordering the phoneme inventory.
    /// apostrophe is used to indicate the beginning of a stressed sylable
    /// must be surounded in slashes: //
    pub fn parse_phonemes(phonemes_str: &str, bank: &PhonemeBank) -> Result<Self> {
        // remove all whitespace
        let input: String = phonemes_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let Some(input) = input.strip_prefix('/') else {
            return Err(Error::PhonemeSymbolParsingError(format!(
                "Phoneme must begin with slash: {}",
                phonemes_str
            )));
        };
        let Some(input) = input.strip_suffix('/') else {
            return Err(Error::PhonemeSymbolParsingError(format!(
                "Phoneme must end with slash: {}",
                phonemes_str
            )));
        };

        let mut remaining_input: &str = &input;
        // index of the first character of the remaining input
        let mut index = 0;
        let mut result = Self {
            phonemes: vec![],
            sylables: vec![],
        };

        let mut cur_sylable = PhonemeStringSylable {
            start: 0,
            end: 0,
            stressed: false,
        };

        while !remaining_input.is_empty() {
            // parse a sylable maker (. or ')
            let stressed_syl = remaining_input.starts_with('\'');
            let unstressed_syl = remaining_input.starts_with('.');
            if stressed_syl || unstressed_syl {
                // record the last sylable
                if cur_sylable.start < cur_sylable.end {
                    result.sylables.push(cur_sylable.clone());
                }
                // start the next sylable
                cur_sylable = PhonemeStringSylable {
                    start: index,
                    end: index,
                    stressed: stressed_syl,
                };
                remaining_input = &remaining_input[1..];
                continue;
            }
            index += 1;
            cur_sylable.end = index;
            // the phoneme symbol that matches is chosen
            let mut found = false;
            for (id, phoneme) in &bank.phonemes {
                let sym = &phoneme.symbol;
                if remaining_input.starts_with(sym) {
                    // first match
                    result.phonemes.push(*id);
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

        result.sylables.push(cur_sylable);
        Ok(result)
    }

    pub fn format_phonemes(&self, _bank: &PhonemeBank) -> String {
        todo!()
    }
}
