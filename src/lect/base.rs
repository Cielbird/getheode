use std::vec;

use crate::phoneme::{PhonemeBank, PhonemeId};
use crate::phonological_rule::PhonologicalRule;
use crate::phonotactics::Phonotactics;
use crate::segment::SegmentString;

/// Defines the phonology of a single way of speaking
pub struct Lect {
    _phonemes: PhonemeBank,

    /// The context-free grammar dictating how phonemes can be arranged
    _phonotactics: Phonotactics,

    /// rules that map the underlying representation to the realized sound segments
    _realization_rules: Vec<PhonologicalRule>,
}

impl Lect {
    pub fn new() -> Self {
        Self {
            _phonemes: PhonemeBank::new(),
            _phonotactics: Phonotactics::new(),
            _realization_rules: vec![],
        }
    }

    pub fn validate_word(&self, _word: Vec<PhonemeId>) -> bool {
        unimplemented!();
    }

    pub fn get_surf_rep(&self, _phonemes: Vec<PhonemeId>) -> SegmentString {
        unimplemented!();
    }
}
