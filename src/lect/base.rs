use std::vec;

use crate::phoneme::{PhonemeBank, PhonemeId};
use crate::phonological_rule::PhonologicalRule;
use crate::segment::SegmentString;
use crate::phonotactics::Phonotactics;

// a lect is a (human) way of speaking.

// contains an inventory of phonemes, phonotactic rules, and a set of realization rules.
// 1) phoneme inventory
//      the set of phonemes that can be used
// 2) phonotactic rules
//      set of rules that define how phonemes can be put together
// 3) realization rules
//      set of rules that define how a sequence of phonemes (called an underlying representation)
//      will be realized as sounds
//
//lects can differ between people, places, social circles, etc.
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

