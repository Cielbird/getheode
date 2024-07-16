use std::fmt;

#[derive(Debug, Clone)]
pub enum GetheodeError {
    /// param is the ipa symbol that couldn't be identified
    IPASymbolParsingError(String),
    /// param is the unknown feature name
    UnknownFeatureName(String),
    /// param is the erronious string
    SegmentParsingError(String),
    /// param is the erronious string
    SegmentStringParsingError(String),
    /// param is the erronious string
    PhonologicalRuleParsingError(String),
    
    RepresentationParsingError(String),
}

impl fmt::Display for GetheodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GetheodeError::IPASymbolParsingError(s) => {
                write!(f, "Unknown IPA symbol: {}", s)
            }
            GetheodeError::UnknownFeatureName(s) => {
                write!(f, "Unknown feature name: {}", s)
            }
            GetheodeError::SegmentParsingError(s) => {
                write!(
                    f,
                    "Unable to parse the following string for a Segment: {}",
                    s
                )
            }
            GetheodeError::SegmentStringParsingError(s) => {
                write!(
                    f,
                    "Unable to parse the following string for a SegmentString: {}",
                    s
                )
            }
            GetheodeError::PhonologicalRuleParsingError(s) => {
                write!(f, "Unable to parse the following string for a rule: {}", s)
            }
            GetheodeError::RepresentationParsingError(s) => {
                write!(f, "representation parsing error: {}", s)
            }
        }
    }
}
