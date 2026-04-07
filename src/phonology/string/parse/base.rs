use nom::error::ParseError;

use nom::{PResult, Parser};

use crate::phonology::string::PhonoString;

struct PhonoStringParser {}

enum PhonoStringParseError {}

impl<'a> ParseError<&'a str> for PhonoStringParseError {
    fn from_error_kind(input: &'a str, kind: nom::error::ErrorKind) -> Self {
        todo!()
    }

    fn append(input: &'a str, kind: nom::error::ErrorKind, other: Self) -> Self {
        todo!()
    }
}

impl<'a> Parser<&'a str> for PhonoStringParser {
    type Output = PhonoString;

    type Error = PhonoStringParseError;

    fn process<OM: nom::OutputMode>(
        &mut self,
        input: &'a str,
    ) -> nom::PResult<OM, &'a str, Self::Output, Self::Error> {
        // parse
        // PResult::Ok(())
        todo!()
    }
}

pub fn phono_string_parser<'a>() -> impl Parser<&'a str> {
    PhonoStringParser {}
}
