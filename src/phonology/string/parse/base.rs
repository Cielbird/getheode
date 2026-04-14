use nom::multi::many1;
use nom::{IResult, Parser};

use crate::d3tree;
use crate::phonology::segment::parse_segment;
use crate::phonology::string::PhonoString;
use crate::phonology::syllable::SyllableFeatures;

/// Parse a phonological string
pub fn parse_phono_string(input: &str) -> IResult<&str, PhonoString> {
    // this parser assumes everything is in the same word, same syllable.
    // TODO add syllable/word boundary parsing
    // TODO build the phono_string struct accordingly

    let mut parser = many1(parse_segment);

    let (remainder, segments) = parser.parse(input)?;

    let mut string = PhonoString::new(d3tree![() => [SyllableFeatures::new_undef() => []]]);
    for seg in segments {
        string.tree.push_depth_2(seg);
    }

    Ok((remainder, string))
}
