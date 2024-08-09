// serves as a struct to represent complete segment strings

use std::fs;

use regex::Regex;

use crate::{errors::GetheodeError, segment::Segment, segment_string::{SegmentString, SegmentStringSlice}};

pub struct Representation {
    symbols: Vec<(SegmentString, String)>
}

impl Representation {
    pub fn empty() -> Self {
        return Representation {
            symbols: Vec::new()
        };
    }

    pub fn from_str(source: &str) -> Result<Self, GetheodeError> {
        let mut symbols = Vec::new();
        for (i, line) in source.split("\n").enumerate() {
            let line = line.trim();
            if line == "" {
                continue;
            }
            
            let p = r"^([^(?:\/\/)]+):?([^(?:\/\/)]+)?";
            let re = Regex::new(p).unwrap();
            if let Some(capts) = re.captures(line) {
                // string representation
                let rep = capts[1].to_string().trim().to_string();
                // segmentstring symbol
                let symbol: SegmentString;
                match capts.get(2) {
                    Some(cpt_2) => {
                        let symbol_as_ipa = cpt_2.as_str().trim();
                        match SegmentString::new(symbol_as_ipa) {
                            Ok(s) => symbol = s,
                            Err(e) => return Err(e)
                        }
                    }
                    None => {
                        match SegmentString::new(&rep) {
                            Ok(s) => symbol = s,
                            Err(e) => return Err(e)
                        }
                    }
                }
                symbols.push((symbol, rep));
            } else {
                let msg = format!("line {}, illegal syntax: {}", i+1, line);
                return Err(GetheodeError::RepresentationParsingError(msg));
            }
        }
        return Ok(Representation {
            symbols: symbols
        });
    }

    pub fn to_rep(&self, string: &SegmentString) -> Result<String, GetheodeError> {
        // we will try to the symbols in a string based on the symbol library
        // provided in the representation struct.
        let mut rep_vec: Vec<String> = Vec::new();
        return self.to_rep_rec(string.slice_all(), &mut rep_vec);
    }

    /// recursive function that parses the symbols used in a `representation` string. 
    /// `symbols` parameter is the wip list of symbols that will be returned.
    fn to_rep_rec(&self, string: SegmentStringSlice, rep_vec: &mut Vec<String>) -> Result<String, GetheodeError> {
        if string.is_empty() {
            // clone and concatenate all the strings and return it
            let mut result = "".to_string();
            for rep in rep_vec {
                result.push_str(rep);
            }
            return Ok(result);
        }
        let str_len = string.len();
        for end in (0..str_len).rev() {
            for (seg, sym) in &self.symbols {
                let s = string.slice(0, end);
                if s == seg.slice_all() {
                    rep_vec.push(sym.clone());
                    match self.to_rep_rec(string.slice(end, str_len), rep_vec) {
                        Ok(segs) => {
                            return Ok(segs)
                        }
                        Err(e) => {
                            rep_vec.pop();
                        }
                    }
                }
            }
        }
        return Err(GetheodeError::RepresentationParsingError("unimplemented error string".to_string()));
    }

    /// converts a string of representation symbols to a segment string
    pub fn from_rep(&self, representation: &str) -> Result<SegmentString, GetheodeError> {
        // we will try to the symbols in a string based on the symbol library
        // provided in the representation struct.
        let mut seg_string_vec: Vec<SegmentString> = Vec::new();
        let ret = self.from_rep_rec(representation, &mut seg_string_vec);
        return ret;
    }

    /// recursive function that parses the symbols used in a `representation` string. 
    /// `symbols` parameter is the wip list of symbols that will be returned.
    fn from_rep_rec(&self, representation: &str, seg_string_vec: &mut Vec<SegmentString>) -> Result<SegmentString, GetheodeError> {
        if representation == "" {
            // clone and concatenate all the segmentstrings and return it
            let mut result = SegmentString::empty();
            for seg_string in seg_string_vec {
                result.append(seg_string.clone());
            }
            return Ok(result);
        }
        // regularize the representation: avoid utf8 shenanegans
        // TODO reenable
        //let representation = &representation.nfd().collect::<String>();

        let rep_len =representation.len();
        for end in (1..=rep_len).rev() {
            let s;
            // try to get substring in byte range
            match representation.get(0..end) {
                Some(substr) => s = substr,
                // fails if `end` is at char boundary
                None => continue
            }
            for (seg, sym) in &self.symbols {
                // regularize the symbol string: no utf8 shenanegans
                // TODO reenable
                //let sym = &sym.nfd().collect::<String>();
                if s.to_owned() == *sym {
                    seg_string_vec.push(seg.clone());
                    match self.from_rep_rec(&representation[end..rep_len], seg_string_vec) {
                        Ok(segs) => {
                            return Ok(segs)
                        }
                        Err(e) => { 
                            seg_string_vec.pop();
                        }
                    }
                }
            }
        }
        println!("rep before error: {}", representation);
        return Err(GetheodeError::RepresentationParsingError("Couldn't parse representation!".to_string()));
    }
}
