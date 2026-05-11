use crate::phonology::feature::FeatureState;
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
use nom::sequence::preceded;

/// Parse a segment element in a phonological rule
/// like parse_segment, but tags can be added: C_1 means a consonant, with segment tagged "1"
fn parse_segment_elem(input: &str) -> IResult<&str, Element> {
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
    let parser = one_of("#$.");

    let mut parser = map(parser, |x| {
        if x == '#' {
            Element::WordBoundary
        } else {
            Element::SyllableBoundary
        }
    });

    parser.parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
pub fn parse_rule_elem(input: &str) -> IResult<&str, Element> {
    let mut parser = alt((parse_segment_elem, parse_bound_elem));

    parser.parse(input)
}

/// parse a sequence of segments or boundaries ex: "es#ma.tan"
///
/// A `'` before a syllable marks it as stressed: all segments in that syllable
/// receive `SyllableFeatures::new([POS])` in their SyllableInfo. The flag resets
/// at every subsequent boundary.
pub fn parse_rule_elems(input: &str) -> IResult<&str, ElementSequence> {
    // empty sequence is a null symbol
    if let Ok((rest, _)) = one_of::<_, _, nom::error::Error<&str>>("∅Ø")(input) {
        return Ok((rest, ElementSequence::new(vec![])));
    }

    let mut elements: Vec<Element> = vec![];
    let mut remaining = input;
    let mut syl_stressed = false;

    loop {
        // stressed syllable boundary: sets stress flag for following segments
        if let Ok((rest, _)) = tag::<_, _, nom::error::Error<&str>>("'")(remaining) {
            elements.push(Element::SyllableBoundary);
            syl_stressed = true;
            remaining = rest;
            continue;
        }

        // other boundary: resets stress flag
        if let Ok((rest, elem)) = parse_bound_elem(remaining) {
            elements.push(elem);
            syl_stressed = false;
            remaining = rest;
            continue;
        }

        // segment: apply current syllable stress to its SyllableInfo
        if let Ok((rest, elem)) = parse_segment_elem(remaining) {
            if let Element::Features(mut syl, seg) = elem {
                if syl_stressed {
                    syl.features = SyllableFeatures::new([FeatureState::POS]);
                }
                elements.push(Element::Features(syl, seg));
            }
            remaining = rest;
            continue;
        }

        break;
    }

    if elements.is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many1,
        )));
    }

    Ok((remaining, ElementSequence::new(elements)))
}
