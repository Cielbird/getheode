// serves as a struct to represent complete segment strings

use std::fs;

use regex::Regex;

use crate::{errors::GetheodeError, segment_string::SegmentString};

struct Representation {
    symbols: Vec<(String, SegmentString)>
}

impl Representation {
    pub fn from_file(file: &str) -> Result<Self, GetheodeError> {
        let mut symbols = Vec::new();
        match fs::read_to_string(file) {
            Ok(x) =>  {
                for (i, line) in x.split("\n").enumerate() {
                    if line.trim() == "" {
                        continue;
                    }
                    
                    let p = r"^([^(?:\/\/)]+)\s*:\s*([^(?:\/\/)]+)";
                    let re = Regex::new(p).unwrap();
                    if let Some(capts) = re.captures(line) {
                        let symbol_str = capts[1].to_string();
                        let symbol_val = SegmentString::new(capts[2]);
                        symbols.push((symbol_str, symbol_val));
                    } else {
                        return Err(GetheodeError::RepresentationParsingError(file.to_string()));
                    }
                }
                return Ok(Representation {
                    symbols: symbols
                });
            },
            Err(e) => return Err(GetheodeError::RepresentationParsingError(file.to_string()))
        }
    
    }
}
