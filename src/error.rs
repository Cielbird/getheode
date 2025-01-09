use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IPASymbolParsingError(String),
    UnknownFeatureName(String),
    SegmentParsingError(String),
    SegmentStringParsingError(String),
    PhonologicalRuleParsingError(String),
    RepresentationParsingError(String),
    GBNFParsingError(String),
    YamlSyntaxError(serde_yml::Error),
    YamlFormatError(String),
    Io(std::io::Error),

    Other(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        // TODO change this in the future
        None
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self { 
        return Self::Io(e);
    }
}

impl From<serde_yml::Error> for Error {
    fn from(e: serde_yml::Error) -> Self { 
        return Self::YamlSyntaxError(e);
    }
}
