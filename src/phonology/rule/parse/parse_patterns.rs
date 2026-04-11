use std::iter::zip;

use nom::{
    IResult, Parser as _, branch::alt, bytes::complete::tag, character::complete::{space0, space1}, combinator::{map, opt, recognize, verify}, multi::{many1, separated_list1}, sequence::{delimited, preceded, separated_pair}
};

use crate::phonology::{rule::{PhonoRuleParseOpts, parse::{elem::{parse_bound_elem, parse_rule_elem}, pattern::{Pattern, RulePatterns}}}, segment::{parse_ipa_base, parse_ipa_diacritic, parse_natural_class, parse_segment_feature_set}};

pub fn parse_rule_patterns(
    rule: &str,
    _opts: PhonoRuleParseOpts,
) -> IResult<&str, RulePatterns<'_>> {
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

    let mut rule = RulePatterns {
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

pub fn parse_rule_elem_branch(input: &str) -> IResult<&str, Pattern<'_>> {
    let parser = delimited(
        tag("{"),
        separated_list1(tag(","), delimited(space0, parse_rule_pattern, space0)),
        tag("}"),
    );
    let mut parser = map(parser, Pattern::branch);

    parser.parse(input)
}

fn parse_rule_elem_opt(input: &str) -> IResult<&str, Pattern<'_>> {
    let parser = delimited(tag("("), parse_rule_pattern, tag(")"));
    let mut parser = map(parser, |tree| {
        // optional is just a branch between `tree` and `null`
        Pattern::optional(tree)
    });

    parser.parse(input)
}

// parse a segment, part of a segment, or boundary.
fn parse_rule_elem_part(input: &str) -> IResult<&str, Pattern<'_>> {
    let part = alt((
        recognize(parse_ipa_base),
        recognize(parse_ipa_diacritic),
        recognize(parse_natural_class),
        recognize(parse_bound_elem),
        delimited(tag("["), recognize(parse_segment_feature_set), tag("]")),
    ));
    let mut parser = map(recognize(many1(part)), Pattern::leaf);

    parser.parse(input)
}

fn parse_rule_elem_null(input: &str) -> IResult<&str, Pattern<'_>> {
    let mut parser = map(tag("Ø"), |_| Pattern::null());
    parser.parse(input)
}

pub fn parse_rule_pattern(input: &str) -> IResult<&str, Pattern<'_>> {
    let mut parser = map(
        many1(alt((
            parse_rule_elem_part,
            parse_rule_elem_branch,
            parse_rule_elem_opt,
            parse_rule_elem_null,
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

fn parse_output(input: &str) -> IResult<&str, &str> {
    let mut parser = recognize(many1(parse_rule_elem));
    parser.parse(input)
}
