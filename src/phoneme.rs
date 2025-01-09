use crate::{error::Result, error::Error::YamlFormatError, segment::Segment, yaml::phoneme_yaml::PhonemeYaml};

/// a struct for representing a phoneme
pub struct Phoneme {
    segment: Segment,
    symbol: String
}

impl Phoneme {
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
}
