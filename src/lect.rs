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

use crate::phonological_rule::PhonologicalRule;
use crate::segment::Segment;

pub struct Lect {
    /// set of underlying representations for each phoneme
    phonemes: Vec<Segment>,

    /// for now, nothing. 
    phonotactic_rules: u8,

    /// rules that map the underlying representation to the realized sound segments
    realization_rules: Vec<PhonologicalRule>
}
