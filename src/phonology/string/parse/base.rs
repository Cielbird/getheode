use nom::combinator::map_res;
use nom::error::ParseError;

use nom::multi::many1;
use nom::{IResult, PResult, Parser};

use crate::phonology::segment::parse_segment;
use crate::phonology::string::PhonoString;
use crate::phonology::syllable::SyllableFeatures;
use crate::ud3tree;

/// Parse a phonological string
pub fn parse_phono_string(input: &str) -> IResult<&str, PhonoString> {
    // this parser assumes everything is in the same word, same syllable.
    // TODO add syllable/word boundary parsing
    // TODO build the phono_string struct accordingly

    let mut parser = many1(parse_segment);

    let (remainder, segments) = parser.parse(input)?;

    let mut string = PhonoString::new(ud3tree![() => [SyllableFeatures::new_undef() => []]]);
    for seg in segments {
        string.tree.layer_2.push((seg, 0));
    }

    Ok((remainder, string))
}
