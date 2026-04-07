use crate::phonology::rule::{PhonoRuleParseOpts, PhonoRuleSet};
use nom::branch::alt;
use nom::combinator::{map, opt, verify};
use nom::error::Error;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::{Err, Parser};
use nom::{IResult, bytes::complete::tag};

// (in -> out, without parentheses or brackets or contexts. A pattern and its replacement)
pub struct StrictRule {
    input: String,
    output: String,
}

pub struct ParsedRule {
    strict_rules: Vec<StrictRule>,
}

fn phono_string<'a>() -> impl Parser<&'a str, Output = &'a str, Error = Error<&'a str>> {
    tag::<&'a str, &'a str, Error<&'a str>>("TODO") // TODO re-visit the phonoSegment parsing to use the nom crate
}

// includes brackets or parentheses for optional/alternatives

fn phono_pattern<'a>() -> impl Parser<&'a str, Output = Vec<&'a str>, Error = Error<&'a str>> {
    alt((
        delimited(
            tag("{"),
            separated_list1(tag(","), phono_string()),
            tag("}"),
        ),
        map(phono_string(), |x| vec![x]),
    ))
}

pub fn parse_rule(rule: &str, opts: PhonoRuleParseOpts) -> IResult<&str, PhonoRuleSet> {
    let input = separated_list1(tag(" "), phono_pattern());
    let output = separated_list1(tag(" "), phono_string());
    let inner_rule = (input, tag("->"), output);
    let context = preceded(tag("/"), (phono_pattern(), tag("_"), phono_pattern()));
    let parser = (inner_rule, opt(context));
    // number of input choices should match number of output choices
    let mut parser_verified = verify(parser, |((input, _, output), _)| {
        input.len() == output.len()
    });

    let (remainder, parsed) = parser_verified.parse(rule)?;

    // TODO this needs to be used to generate all the strict phonological rules
    let ((input, _, output), context) = parsed;
    if let Some((pre_context, _, post_context)) = context {}

    Ok((remainder, PhonoRuleSet::new()))
}
