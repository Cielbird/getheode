use std::io::Write;
use std::iter::zip;

use crate::phonology::rule::parse::parsed::{ParsedRule, PhonoRuleSet};
use crate::phonology::rule::parse::synthesis::enumerate_branches;
use crate::phonology::rule::parse::tree::{ParsedRuleElem, ParsedRulePattern};
use crate::phonology::rule::{PhonoRuleParseOpts, SegmentInfo, SyllableInfo};
use crate::phonology::segment::parse_segment;
use crate::phonology::syllable::SyllableFeatures;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of, space0};
use nom::combinator::{map, map_res, opt, verify};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair};

/// Parse a segment element in a phonological rule
/// like parse_segment, but tags can be added: C_1 means a consonant, with segment tagged "1"
fn parse_segment_elem(input: &str) -> IResult<&str, ParsedRuleElem> {
    // TODO add parsing for syllable features
    // ex: C[+high-stress]
    let parser = (
        parse_segment,
        opt(preceded(tag("_"), map_res(digit1, str::parse))),
    );
    let mut parser = map(parser, |(seg_features, tag)| {
        ParsedRuleElem::Features(
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

fn parse_bound_elem(input: &str) -> IResult<&str, ParsedRuleElem> {
    const SYL_BOUNDARIES: &[char] = &['$', '.'];
    const WORD_BOUNDARIES: &[char] = &['#'];
    let parser = one_of("#$.");

    let mut parser = map(parser, |x| {
        if SYL_BOUNDARIES.contains(&x) {
            return ParsedRuleElem::SyllableBoundary;
        } else {
            return ParsedRuleElem::WordBoundary;
        }
    });

    parser.parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
fn parse_null_elem(input: &str) -> IResult<&str, ParsedRuleElem> {
    map(tag("∅"), |_| ParsedRuleElem::Null).parse(input)
}

/// Parse a elem which may be tagged segment, or boundary.
fn parse_rule_elem(input: &str) -> IResult<&str, ParsedRuleElem> {
    println!("elem: {input}");
    let mut parser = alt((parse_segment_elem, parse_bound_elem, parse_null_elem));

    parser.parse(input)
}

fn parse_rule_elem_branch(input: &str) -> IResult<&str, ParsedRulePattern> {
    let parser = delimited(
        tag("{"),
        separated_list1(tag(","), parse_rule_tree),
        tag("}"),
    );
    let mut parser = map(parser, |trees| ParsedRulePattern::branch(trees));

    parser.parse(input)
}

fn parse_rule_elem_opt(input: &str) -> IResult<&str, ParsedRulePattern> {
    let parser = delimited(tag("("), parse_rule_tree, tag(")"));
    let mut parser = map(parser, |tree| {
        // optional is just a branch between `tree` and `null`
        ParsedRulePattern::branch(vec![tree, ParsedRulePattern::null()])
    });

    parser.parse(input)
}

fn parse_rule_tree(input: &str) -> IResult<&str, ParsedRulePattern> {
    println!("tree: {input}");
    let mut parser = map(
        many1(alt((
            map(parse_rule_elem, |e| ParsedRulePattern::leaf(e)),
            parse_rule_elem_branch,
            parse_rule_elem_opt,
        ))),
        |x| ParsedRulePattern::sequence(x),
    );

    parser.parse(input)
}

pub fn parse_rule(rule: &str, opts: PhonoRuleParseOpts) -> IResult<&str, ParsedRule> {
    let input = separated_list1(tag(" "), parse_rule_tree);
    // output has no branching
    let output = separated_list1(tag(" "), many1(parse_rule_elem));
    let inner_rule = separated_pair(input, delimited(space0, tag("->"), space0), output);
    let context = preceded(
        tag("/"),
        separated_pair(opt(parse_rule_tree), tag("_"), opt(parse_rule_tree)),
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

    let mut rule = ParsedRule {
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
