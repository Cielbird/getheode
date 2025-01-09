pub mod diacritics;
pub mod error;
pub mod feature;
pub mod ipa_segments;
pub mod natural_classes;
pub mod phonological_rule;
pub mod segment;
pub mod segment_string;
pub mod phoneme;
pub mod lect;

pub mod gbnf;
pub mod yaml;

pub const GETHEODE_VERSION: &str = env!("CARGO_PKG_VERSION");
