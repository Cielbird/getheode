use std::rc::Rc;
use std::vec;

use crate::error::{Error, Error::PhonemeSymbolParsingError, Result};
use crate::gbnf::grammar::{self, Grammar};
use crate::phonological_rule::PhonologicalRule;
use crate::phoneme::Phoneme;
use crate::yaml::lect_yaml::LectYaml;

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
    /// set of phonemes used and their data
    phonemes: Vec<Rc<Phoneme>>,

    /// The formal grammar dictating how phonemes can be arranged
    phonotactics: Grammar,

    /// rules that map the underlying representation to the realized sound segments
    realization_rules: Vec<PhonologicalRule>
}

impl Lect {
    pub fn from_yaml(file_path: &str) -> Result<Lect>  {
        // parse yaml text to objects
        let yaml = LectYaml::from_file(file_path)?;
        
        // process phonemes
        let mut phoneme_vec = vec![];
        for phoneme_yaml in yaml.phonemes {
            phoneme_vec.push(Rc::new(Phoneme::from_yaml(phoneme_yaml)?));
        }

        // process and parse phonotactics gbnf
        let phonotactics = Grammar::from_productions(yaml.phonotactics, &phoneme_vec)?;

        // process and parse phonological realization rules
        let mut rules = vec![];
        for rule in yaml.phonological_rules {
            rules.push(PhonologicalRule::from_string(&rule)?);
        }

        // create the lect
        let lect = Lect { 
            phonemes: phoneme_vec,
            phonotactics: phonotactics,
            realization_rules: rules };

        return Ok(lect);
    }

    pub fn validate_word(&self, phonemes: &str) -> bool{
        return false;
    }
}
