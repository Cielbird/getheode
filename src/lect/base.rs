use std::vec;

use crate::error::*;
use crate::phoneme::{FormatPhonemes, PhonemeBank, PhonemeId};
use crate::phonological_rule::PhonologicalRule;
use crate::phonotactics::Phonotactics;
use crate::segment::SegmentString;

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
    pub fn get_surf_rep(&self, phonemes: Vec<PhonemeId>) -> SegmentString {
        let mut string = self.phonemes.underlying_rep(phonemes);
        for rule in &self.realization_rules {
            string = rule.apply(&string).unwrap();
        }

        string
    }
}

impl FormatPhonemes for Lect {
    fn parse_phonemes(&self, phonemes_str: &str) -> Result<Vec<PhonemeId>> {
        self.phonemes.parse_phonemes(phonemes_str)
    }

    fn format_phonemes(&self, phonemes: Vec<PhonemeId>) -> String {
        self.phonemes.format_phonemes(phonemes)
    }
}
