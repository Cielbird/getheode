use crate::phonology::rule::parse::elem::{Element, ElementSequence};
use crate::phonology::rule::{SegmentInfo, SyllableInfo};
use crate::phonology::segment::parse_segment;
use crate::phonology::syllable::SyllableFeatures;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{map, map_res, opt};
use nom::multi::many1;
use nom::sequence::preceded;

/// Parse a segment element in a phonological rule
/// like parse_segment, but tags can be added: C_1 means a consonant, with segment tagged "1"
fn parse_segment_elem(input: &str) -> IResult<&str, Element> {
    // TODO add parsing for syllable features
    // ex: C[+high-stress]
    let parser = (
        parse_segment,
        opt(preceded(tag("_"), map_res(digit1, str::parse))),
    );
    let mut parser = map(parser, |(seg_features, tag)| {
        Element::Features(
            SyllableInfo {
                tag: None,
                features: SyllableFeatures::new_undef(),
            },
            SegmentInfo {
                tag,
                features: seg_features,
            },
        )
    });

    parser.parse(input)
}

pub fn parse_bound_elem(input: &str) -> IResult<&str, Element> {
    const SYL_BOUNDARIES: &[char] = &['$', '.'];
    const _WORD_BOUNDARIES: &[char] = &['#'];
    let parser = one_of("#$.");

    let mut parser = map(parser, |x| {
        if SYL_BOUNDARIES.contains(&x) {
            Element::SyllableBoundary
        } else {
            Element::WordBoundary
        }
    });

    parser.parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
pub fn parse_rule_elem(input: &str) -> IResult<&str, Element> {
    let mut parser = alt((parse_segment_elem, parse_bound_elem));

    parser.parse(input)
}

pub fn parse_rule_elems(input: &str) -> IResult<&str, ElementSequence> {
    let mut parser = map(many1(parse_rule_elem), ElementSequence::new);

    parser.parse(input)
}
