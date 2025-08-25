use crate::error::{Error, Result};

/// a feature of a phonological segment

// these constants exis because rust enums are nothing like C enums. unfortunate discovery.
// MAJOR CLASS
pub const SYL: u8 = 0;
pub const STRESS: u8 = 1;
pub const LONG: u8 = 2;
pub const CONS: u8 = 3;
pub const SON: u8 = 4;
// MANNER
pub const CONT: u8 = 5;
pub const DELREL: u8 = 6;
pub const APPROX: u8 = 7;
pub const TAP: u8 = 8;
pub const TRILL: u8 = 9;
pub const NASAL: u8 = 10;
// LARYNGEAL
pub const VOI: u8 = 11;
pub const SPGL: u8 = 12;
pub const CONGL: u8 = 13;
// PLACE
// LABIAL
pub const LAB: u8 = 14;
pub const ROUND: u8 = 15;
pub const LABDENT: u8 = 16;
// CORONAL
pub const COR: u8 = 17;
pub const ANT: u8 = 18;
pub const DIST: u8 = 19;
pub const STRIDENT: u8 = 20;
pub const LATERAL: u8 = 21;
// DORSAL
pub const DOR: u8 = 22;
pub const HIGH: u8 = 23;
pub const LOW: u8 = 24;
pub const FRONT: u8 = 25;
pub const BACK: u8 = 26;
pub const TENSE: u8 = 27;

pub const FEATURE_COUNT: u8 = 28;

pub const FEATURE_NAMES: [&str; FEATURE_COUNT as usize] = [
    "syl", "stress", "long", "cons", "son", "cont", "delrel", "approx", "tap", "trill", "nasal",
    "voi", "spgl", "congl", "lab", "round", "labdent", "cor", "ant", "dist", "strident", "lateral",
    "dor", "high", "low", "front", "back", "tense",
];

pub type Feature = u8;

/// converts a feature name string to the corresponding u8 index
pub fn feature_from_string(string: &str) -> Result<Feature> {
    let index = FEATURE_NAMES.iter().position(|s| *s == string);
    match index {
        Some(i) => Ok(i as u8),
        None => Err(Error::UnknownFeatureName(string.to_string())),
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, Hash, PartialEq)]
pub enum FeatureState {
    #[default]
    UNDEF, // undefined for this segment. this indicates the segment is incomplete.
    POS, // (+) present in the segment
    NEG, // (-) not present in the segment
    NA,  // not applicable, could be either positive or negative for this segment, we don't care
}

// 28 features, each has 4 possible values: fixed width struct
// 56 bits per segment, 7 bytes
