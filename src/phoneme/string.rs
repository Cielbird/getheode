use crate::phoneme::{PhonemeBank, PhonemeId};

#[derive(Debug, PartialEq, Eq)]
pub struct PhonemeString {
    pub(crate) phonemes: Vec<PhonemeId>,
    pub(crate) sylables: Vec<PhonemeStringSylable>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhonemeStringSylable {
    /// inclusive
    pub(crate) start: usize,
    /// exclusive
    pub(crate) end: usize,
    // could be later expanded to include more sylable-level features
    pub(crate) stressed: bool,
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
