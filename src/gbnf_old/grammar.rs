use std::rc::Rc;

use super::production::Production;
use crate::{error::{Error, Result}, gbnf::term::Term::{NonTerminal, None, Terminal}, phoneme::Phoneme};
use rand::Rng;

pub struct Grammar {
    pub productions: Vec<Production>
}

impl Grammar {
    /// Parses a vector of gbnf strings and constructs a new Grammar.
    pub fn from_productions(inputs: Vec<String>, phoneme_inv: &Vec<Rc<Phoneme>>) -> Result<Grammar> {
        let mut productions: Vec<Production> = Vec::new();

        for prod in inputs {
            let prod = prod.trim();

            // Skip empty lines or comments
            if prod.is_empty() || prod.starts_with("//") {
                continue;
            }

            productions.push(Production::from_string(prod, phoneme_inv)?);
        }

        Ok(Grammar { productions })
    }

    /// generates a random segment string with the grammar
    /// grammar must contain a <word> non-terminal, which is used as the root of the word.
    /// all OR choices (seperated by |) are given equal probability. 
    pub fn generate_random_word(&self) -> Result<Vec<Rc<Phoneme>>> {
        return self.generate_random("word");
    }

    /// generates a random segment string with the grammar using a given non-terminal as root.
    /// all OR choices (seperated by |) are given equal probability. 
    pub fn generate_random(&self, root: &str) -> Result<Vec<Rc<Phoneme>>> {
        for prod in &self.productions {
            if prod.lhs == root {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..prod.rhs.len()); // Generate random index
                
                let mut final_seg_str: Vec<Rc<Phoneme>> = vec![];
                for e in &prod.rhs[random_index].terms {
                    match e {
                        Terminal(seg_str) => {
                            final_seg_str.push(seg_str.clone());
                        }
                        NonTerminal(str) => {
                            let recursive_result = self.generate_random(&str)?;
                            final_seg_str.extend(recursive_result);
                        }
                        None => continue
                    }
                }
                return Ok(final_seg_str);
            }
        }
        return Err(Error::Other(format!("Could not find non-terminal \"{}\"", root)));
    }

    pub fn parse_input(&self, input: Vec<Rc<Phoneme>>) -> Result<String> {
        return Ok("asdf".to_string());
    }
}