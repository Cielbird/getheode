use std::{rc::Rc};

use crate::{error::Result, error::Error::{YamlFormatError, PhonemeSymbolParsingError}, segment::Segment, yaml::phoneme_yaml::PhonemeYaml};

/// a struct for representing a phoneme
#[derive(Clone, Debug)] //Deserialize, Serialize, 
pub struct Phoneme {
    pub segment: Segment,
    pub symbol: String
}

impl Phoneme {
    /// Creates a phoneme from a segment and a string
    pub fn new(segment: Segment, symbol: String) -> Self {
        Self {
            segment: segment,
            symbol: symbol
        }
    }

    /// Creates a phoneme from a yaml object. 
    /// if the ipa field is defined, it will define the segment of the phoneme. otherwise, there is
    /// and error.
    /// if the symbol is defined, it will define the symbol field of the phoneme, otherwise, the 
    /// phoneme's symbol will be the ipa of the segment.
    pub fn from_yaml(yaml: PhonemeYaml) -> Result<Self> {
        let segment: Segment;
        match yaml.ipa {
            None => {
                // TODO implement support for xsampa
                return Err(YamlFormatError(format!("phoneme must have an ipa symbol")));
            }
            Some(ipa) => {
                segment = Segment::from_string(&ipa)?;
            }
        }

        let symbol;
        match yaml.symbol {
            None => {
                symbol = segment.to_string();
            }
            Some(s) => {
                symbol = s;
            }
        }

        let phoneme = Self {
            segment: segment,
            symbol: symbol
        };
        return Ok(phoneme);
    }

    /// parses a string and identifies the sequence of phonemes used
    /// the phoneme symbols are used to identify them.
    /// first match found is used, so if there are issues with findinf phonemes, 
    /// consider reordering the phoneme inventory.
    pub fn parse_phonemes(input: &str, phoneme_inv: &Vec<Rc<Phoneme>>) -> Result<Vec<Rc<Phoneme>>> {
        // remove all whitespace
        let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
        
        let mut remaining_input: &str = &input;
        let mut result: Vec<Rc<Phoneme>> = vec![];

        while !remaining_input.is_empty() {
            // the phoneme symbol that matches is chosen
            let mut found = false;
            for phoneme in phoneme_inv {
                let sym = &phoneme.symbol;
                let len = sym.len();
                if remaining_input[0..len] == *sym {
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
