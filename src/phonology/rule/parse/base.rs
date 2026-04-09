use std::iter::zip;

use crate::phonology::rule::{PhonoRule, PhonoRuleParseOpts, PhonoRuleSet};
use crate::phonology::segment::{SegmentFeatures, parse_segment};
use crate::phonology::string::{PhonoString, parse_phono_string};
use nom::branch::alt;
use nom::combinator::{map, opt, verify};
use nom::error::Error;
use nom::multi::{many1, separated_list1};
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

// includes brackets or parentheses for optional/alternatives

fn phono_pattern<'a>() -> impl Parser<&'a str, Output = Vec<PhonoString>, Error = Error<&'a str>> {
    alt((
        delimited(
            tag("{"),
            separated_list1(tag(","), parse_phono_string),
            tag("}"),
        ),
        map(parse_phono_string, |x| vec![x]),
    ))
}

pub fn parse_rule(rule: &str, opts: PhonoRuleParseOpts) -> IResult<&str, PhonoRuleSet> {
    let input = separated_list1(tag(" "), phono_pattern());
    let output = separated_list1(tag(" "), parse_phono_string);
    let inner_rule = (input, tag("->"), output);
    let context = preceded(
        tag("/"),
        (opt(phono_pattern()), tag("_"), opt(phono_pattern())),
    );
    let parser = (inner_rule, opt(context));
    // number of input choices should match number of output choices
    let mut parser_verified = verify(parser, |((input, _, output), _)| {
        input.len() == output.len()
    });

    let (remainder, parsed) = parser_verified.parse(rule)?;

    let ((input, _, output), context) = parsed;
    if let Some((pre_context, _, post_context)) = context {}

    let mut rules = vec![];
    for (input_opts, output) in zip(input, output) {
        for input in input_opts {
            // TODO need to assign ids to different segments, in a smart way. this task needs
            // formal definition and a dedicated algorithm
            let match_tree = todo!();
            let replace_tree = todo!();
            let rule = PhonoRule {
                pattern: match_tree,
                replace_tree,
            };
            rules.push(rule);
        }
    }

    // TODO include context in rules

    Ok((remainder, PhonoRuleSet::new(rule.to_string(), rules)))
}
