use serde::{Serialize, Deserialize};

/// a struct for use in serialization/deserialization of phonemes (yaml files for example)
#[derive(Debug, Serialize, Deserialize)]
pub struct PhonemeYaml {
    pub ipa: Option<String>,
    pub symbol: Option<String>,
    pub xsampa: Option<String>
}
