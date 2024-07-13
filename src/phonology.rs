use crate::phonological_rule::PhonologicalRule;

pub struct Phonology {
    phonemes: Vec<String>,
    rules: Vec<PhonologicalRule>,
    phonotactics: String
}
