use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Other(String), // most errors in the getheode engine are this
    Io(std::io::Error),
}
impl Error {
    pub fn other<T>(text: T) -> Error
    where
        T: fmt::Display,
    {
        Self::Other(text.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Other(text) => write!(f, "Getheode: {text}"),
            Error::Io(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Other(_) => None,
            Error::Io(error) => Some(error),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
