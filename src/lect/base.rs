use std::vec;

use crate::error::*;
use crate::phoneme::{PhonemeBank, PhonemeId, PhonemeString};
use crate::phonological_rule::PhonologicalRule;
use crate::phonotactics::Phonotactics;
use crate::segment::PhonologicalString;

/// Defines the phonology of a single way of speaking
pub struct Lect {
    phonemes: PhonemeBank,

    /// The context-free grammar dictating how phonemes can be arranged
    _phonotactics: Phonotactics,

    /// rules that map the underlying representation to the realized sound segments
    realization_rules: Vec<PhonologicalRule>,
}

impl Lect {
    pub fn empty() -> Self {
        Self {
            phonemes: PhonemeBank::new(),
            _phonotactics: Phonotactics::new(),
            realization_rules: vec![],
        }
    }

    pub fn new(
        phonemes: PhonemeBank,
        phonotactics: Phonotactics,
        realization_rules: Vec<PhonologicalRule>,
    ) -> Self {
        Self {
            phonemes,
            _phonotactics: phonotactics,
            realization_rules,
        }
    }

    pub fn validate_word(&self, _word: Vec<PhonemeId>) -> bool {
        unimplemented!();
    }

    /// Get the surface representation of a sequence of phonemes
    pub fn get_surf_rep(&self, phonemes: PhonemeString, worded: bool) -> PhonologicalString {
        let mut string = self.phonemes.underlying_rep(phonemes);
        if worded {
            string = string.worded();
        }
        for rule in &self.realization_rules {
            string = rule.apply(&string).unwrap();
        }

        string
    }

    pub fn parse_phonemes(&self, phonemes_str: &str) -> Result<PhonemeString> {
        PhonemeString::parse_phonemes(phonemes_str, &self.phonemes)
    }

    pub fn format_phonemes(&self, phonemes: PhonemeString) -> String {
        PhonemeString::format_phonemes(&phonemes, &self.phonemes)
    }
}
