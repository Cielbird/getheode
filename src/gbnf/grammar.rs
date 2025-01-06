use std::fmt::format;

use super::{expression::Expression, production::Production};
use crate::{gbnf::term::Term::{NonTerminal, Terminal}, segment_string::SegmentString};
use rand::Rng;

pub struct Grammar {
    pub productions: Vec<Production>
}

impl Grammar {
    /// Parses a gbnf string and constructs a new Grammar.
    pub fn parse_bnf(input: &str) -> Result<Grammar, String> {
        let mut productions = Vec::new();

        for line in input.lines() {
            let line = line.trim();

            // Skip empty lines or comments
            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            // Split line into lhs and rhs
            let parts: Vec<&str> = line.split("::=").map(str::trim).collect();
            if parts.len() != 2 {
                return Err(format!("Invalid production: {}", line));
            }

            let mut lhs = parts[0].trim().to_string();
            if !lhs.starts_with('<') || !lhs.ends_with('>') {
                return Err(format!("Invalid non-terminal: {}", lhs));
            }
            // trim angle brackets from lhs
            lhs = lhs.trim_matches(|c| c == '<' || c == '>').to_string();

            let rhs = parts[1].trim();
            let expressions = Expression::parse_expressions(rhs)?;

            productions.push(Production { lhs, rhs: expressions });
        }

        Ok(Grammar { productions })
    }

    /// generates a random segment string with the grammar
    /// grammar must contain a <word> non-terminal, which is used as the root of the word.
    /// all OR choices (seperated by |) are given equal probability. 
    pub fn generate_random_word(&self) -> Result<SegmentString, String> {
        return self.generate_random("word");
    }

    /// generates a random segment string with the grammar using a given non-terminal as root.
    /// all OR choices (seperated by |) are given equal probability. 
    pub fn generate_random(&self, root: &str) -> Result<SegmentString, String> {
        for prod in &self.productions {
            if prod.lhs == root {
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..prod.rhs.len()); // Generate random index
                
                let mut final_seg_str = SegmentString::new("").unwrap();
                for e in &prod.rhs[random_index].terms {
                    match e {
                        Terminal(seg_str) => {
                            final_seg_str.append(seg_str.clone());
                        }
                        NonTerminal(str) => {
                            let recursive_result = self.generate_random(&str);
                            match recursive_result {
                                Ok(seg_str) => final_seg_str.append(seg_str),
                                Err(e) => return Err(e)
                            }
                        }
                    }
                }
                return Ok(final_seg_str);
            }
        }
        return Err(format!("Could not find non-terminal \"{}\"", root));
    }
}