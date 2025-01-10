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

        // process and parse phonological realization rules
        let mut rules = vec![];
        for rule in yaml.phonological_rules {
            rules.push(PhonologicalRule::from_string(&rule)?);
        }

        // create the lect
        let mut lect = Lect { 
            phonemes: phoneme_vec,
            phonotactics: Grammar{productions: vec![]},
            realization_rules: rules };

        // process and parse phonotactics gbnf
        lect.phonotactics = Grammar::from_productions(yaml.phonotactics, &lect)?;

        return Ok(lect);
    }

    /// parses a string and identifies the sequence of phonemes used
    /// the phoneme symbols are used to identify them.
    /// first match found is used, so if there are issues with findinf phonemes, 
    /// consider reordering the phoneme inventory.
    pub fn parse_phonemes(&self, input: &str) -> Result<Vec<Rc<Phoneme>>> {
        // remove all whitespace
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
        
        let mut remaining_input: &str = &input;
        let mut result: Vec<Rc<Phoneme>> = vec![];

        while !input.is_empty() {
            // the phoneme symbol that matches is chosen
            let mut found = false;
            for phoneme in &self.phonemes {
                let sym = &phoneme.symbol;
                let len = sym.len();
                if input[0..len] == *sym {
                    // first match
                    result.push(phoneme.clone());
                    found = true;
                    remaining_input = &remaining_input[len..];
                    break;
                }
            }
            if !found {
                return Err(PhonemeSymbolParsingError(
                    format!("Could not parse the phonemes of the string {}", input)
                ));
            }
        }
        return Ok(result);
    }
}
