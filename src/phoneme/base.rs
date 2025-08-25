use crate::{
    segment::Segment,
};

/// a struct for representing a phoneme
#[derive(Clone, Debug, PartialEq)] //Deserialize, Serialize,
pub struct Phoneme {
    /// The sound / underlying representation of this phoneme
    pub segment: Segment,
    /// Text representation of this phoneme, for user-friendly serialization
    pub symbol: String,
}

impl Phoneme {
    /// Creates a phoneme from a segment and a string
    pub fn new(segment: Segment, symbol: String) -> Self {
        Self { segment, symbol }
    }
}
