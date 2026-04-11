use std::iter::zip;

use crate::phonology::rule::parse::elem::Element;
use crate::phonology::rule::parse::parsed::RulePatternsParsed;
use crate::phonology::rule::parse::pattern::Pattern;
use crate::phonology::rule::{PhonoRuleParseOpts, SegmentInfo, SyllableInfo};
use crate::phonology::segment::{
    parse_ipa_base, parse_ipa_diacritic, parse_natural_class, parse_segment,
    parse_segment_feature_set, parse_segment_ipa,
};
use crate::phonology::syllable::SyllableFeatures;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::{is_not, tag};
use nom::character::complete::{digit1, one_of, space0, space1};
use nom::combinator::{map, map_res, opt, recognize, verify};
use nom::error::dbg_dmp;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};

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
                id: None,
                features: SyllableFeatures::new_undef(),
            },
            SegmentInfo {
                id: tag,
                features: seg_features,
            },
        )
    });

    parser.parse(input)
}

fn parse_bound_elem(input: &str) -> IResult<&str, Element> {
    const SYL_BOUNDARIES: &[char] = &['$', '.'];
    const WORD_BOUNDARIES: &[char] = &['#'];
    let parser = one_of("#$.");

    let mut parser = map(parser, |x| {
        if SYL_BOUNDARIES.contains(&x) {
            return Element::SyllableBoundary;
        } else {
            return Element::WordBoundary;
        }
    });

    parser.parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
fn parse_null_elem(input: &str) -> IResult<&str, Element> {
    map(tag("∅"), |_| Element::Null).parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
pub fn parse_rule_elem(input: &str) -> IResult<&str, Element> {
    let mut parser = alt((parse_segment_elem, parse_bound_elem, parse_null_elem));

    parser.parse(input)
}

pub fn parse_rule_elem_branch(input: &str) -> IResult<&str, Pattern> {
    let parser = delimited(
        tag("{"),
        separated_list1(tag(","), delimited(space0, parse_rule_pattern, space0)),
        tag("}"),
    );
    let mut parser = map(parser, |trees| Pattern::branch(trees));

    parser.parse(input)
}

fn parse_rule_elem_opt(input: &str) -> IResult<&str, Pattern> {
    let parser = delimited(tag("("), parse_rule_pattern, tag(")"));
    let mut parser = map(parser, |tree| {
        // optional is just a branch between `tree` and `null`
        Pattern::optional(tree)
    });

    parser.parse(input)
}

// parse a segment, part of a segment, or boundary.
fn parse_rule_elem_part(input: &str) -> IResult<&str, Pattern> {
    let part = alt((
        recognize(parse_ipa_base),
        recognize(parse_ipa_diacritic),
        recognize(parse_natural_class),
        recognize(parse_natural_class),
        recognize(parse_bound_elem),
        delimited(tag("["), recognize(parse_segment_feature_set), tag("]")),
    ));
    let mut parser = map(recognize(many1(part)), |e| Pattern::leaf(e));

    parser.parse(input)
}

pub fn parse_rule_pattern(input: &str) -> IResult<&str, Pattern> {
    let mut parser = map(
        many1(alt((
            parse_rule_elem_part,
            parse_rule_elem_branch,
            parse_rule_elem_opt,
        ))),
        |mut x| {
            if x.len() == 1 {
                x.remove(0)
            } else {
                Pattern::sequence(x)
            }
        },
    );

    parser.parse(input)
}

fn parse_output(input: &str) -> IResult<&str, Vec<&str>> {
    let mut parser = many1(recognize(parse_rule_elem));
    parser.parse(input)
}

pub fn parse_rule_patterns(
    rule: &str,
    opts: PhonoRuleParseOpts,
) -> IResult<&str, RulePatternsParsed> {
    let input = separated_list1(space1, parse_rule_pattern);
    // output has no branching
    let output = separated_list1(space1, parse_output);
    let inner_rule = separated_pair(input, delimited(space0, tag("->"), space0), output);
    let context = preceded(
        delimited(space0, tag("/"), space0),
        separated_pair(opt(parse_rule_pattern), tag("_"), opt(parse_rule_pattern)),
    );
    let parser = (inner_rule, opt(context));
    // number of input choices should match number of output choices
    let mut parser_verified = verify(parser, |((input, output), _)| input.len() == output.len());

    let (remainder, parsed) = parser_verified.parse(rule)?;

    let ((input, output), context) = parsed;
    let mut pre_context = None;
    let mut post_context = None;
    if let Some((pre, post)) = context {
        pre_context = pre;
        post_context = post;
    }

    let mut rule = RulePatternsParsed {
        input: vec![],
        output: vec![],
        pre_context,
        post_context,
    };
    for (input, output) in zip(input, output) {
        rule.input.push(input);
        rule.output.push(output);
    }

    Ok((remainder, rule))
}
