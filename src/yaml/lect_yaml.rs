use std::fs;
use crate::error::Result;
use serde::{Serialize, Deserialize};
use serde_yml;

use super::phoneme_yaml::PhonemeYaml;



/// a struct for use in serialization/deserialization (yaml files for example)
#[derive(Debug, Serialize, Deserialize)]
pub struct LectYaml {
    pub phonemes: Vec<PhonemeYaml>,
    pub phonotactics: Vec<String>,
    pub phonological_rules: Vec<String>
}

/// a struct for use in serialization/deserialization (yaml files for example)
impl LectYaml {
    pub fn from_file(file_path: &str) -> Result<LectYaml>  {
        // Read the file contents
        let yaml_content = fs::read_to_string(file_path)?;

        // Parse the YAML into the `Data` struct
        let data: LectYaml = serde_yml::from_str(&yaml_content)?;

        Ok(data)
    }
}