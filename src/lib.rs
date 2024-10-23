pub mod diacritics;
pub mod errors;
pub mod feature;
pub mod ipa_segments;
pub mod natural_classes;
pub mod phonological_rule;
pub mod segment;
pub mod segment_string;
pub mod representation;
pub mod lect;

pub const GETHEODE_VERSION: &str = env!("CARGO_PKG_VERSION");
