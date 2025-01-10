// this is the cli tool to interact with the getheode library most directly

use std::fs;

use clap::{Arg, ArgAction, Command};

use getheode;
use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;
use getheode::GETHEODE_VERSION;

/*
fn apply_rules_cli(input_file: &str, input_repr_file: Option<&str>, rules_file: &str, output_repr_file: Option<&str>) {

    let mut in_rep = None;
    if let Some(f) = input_repr_file {
        match fs::read_to_string(f) {
            Ok(x) =>  {
                match Representation::from_str(&x) {
                    Ok(repr) => in_rep = Some(repr),
                    Err(e) => println!("could not read input representations file: \n{e}")
                }
            },
            Err(e) => { println!("rules file {f} is not valid: {e}") }
        }
    }

    let mut inputs = Vec::new();
    match fs::read_to_string(input_file) {
        Ok(x) =>  {
            for (i, segs_str) in x.split("\n").enumerate() {
                let mut trimmed_str = segs_str.trim();
                trimmed_str = remove_comments(trimmed_str);
                if trimmed_str == "" {
                    continue;
                }
                if let Some(ref rep) = in_rep {
                    match rep.from_rep(trimmed_str) {
                        Ok(string) => inputs.push(SegmentString::to_worded(string)),
                        Err(e) => println!("{}:{}, {}",input_file, i+1, e),
                    }
                } else {
                    match SegmentString::new(trimmed_str) {
                        Ok(string) => inputs.push(SegmentString::to_worded(string)),
                        Err(e) => println!("{}:{}, {}",input_file, i+1, e),
                    }
                }
            }
        },
        Err(_) => { println!("rules file {input_file} is not valid") }
    }

    let mut rules = Vec::new();
    match fs::read_to_string(rules_file) {
        Ok(x) =>  {
            for rule_str in x.split("\n") {
                if rule_str.trim() == "" {
                    continue;
                }
                match PhonologicalRule::new(rule_str.trim()) {
                    Ok(rule) => rules.push(rule),
                    Err(e) => println!("invalid rule: {e}"),
                }
            }
        },
        Err(_) => { println!("rules file {rules_file} is not valid") }
    }


    // apply rules
    let mut outputs = Vec::new();
    for input in &inputs {
        let mut output = input.clone();
        for rule in &rules {
            match rule.apply(&output) {
                Ok(out) => output = out,
                Err(e) => println!("rule apply failed: {}",e)
            }
        }
        outputs.push(output);
    }

    if let Some(f) = output_repr_file {
        match fs::read_to_string(f) {
            Ok(x) =>  {
                match Representation::from_str(&x) {
                    Ok(repr) => {
                        for out in outputs {
                            println!("{}", repr.to_rep(&out).unwrap());
                        }
                    },
                    Err(e) => println!("could not read output representations file: {e}")
                }
            },
            Err(e) => { println!("rules file {f} is not valid: {e}") }
        }
    } else {
        for out in outputs {
            println!("{}", out);
        }
    }
}
*/

fn cli() -> Command {
    Command::new("getheode")
        .about("getheode command line tool")
        .version(GETHEODE_VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("valid")
                .about("checks the validity of the words in a lect")
                .arg(
                    Arg::new("lect")
                        .help("expects a .geth file, conforming to getheode's yaml structure")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("input")
                        .help("expects a sequence of morphemes, inside slashes (`/likethis/`). can be either raw input or a file")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("surface")
                .about("gets the surface representation of the words in a lect")
                .arg(
                    Arg::new("lect")
                        .help("expects a .geth file, conforming to getheode's yaml structure")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("input")
                        .help("expects a sequence of morphemes, inside slashes (`/likethis/`). can be either raw input or a file")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            Command::new("gen")
                .about("generates random words according to a lect")
                .arg(
                    Arg::new("lect")
                        .help("expects a .geth file, conforming to getheode's yaml structure")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::new("count")
                        .help("the number of words to generate. must be between 0 and 99, inclusively")
                        .required(true)
                        .index(2)
                        .value_parser(clap::value_parser!(u8).range(0..=99)),
                )
                .arg(
                    Arg::new("surface")
                        .help("if given, the output will be the surface representation of the words")
                        .short('s')
                        .long("surface")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("apply", apply_matches)) => {

            if !apply_matches.contains_id("input") {
                println!("Input file must be supplied!");
                return;
            }
            let input = apply_matches.get_one::<String>("input").unwrap();

            // input rep may or may not exits: we don't care
            let in_rep = apply_matches.get_one::<String>("in_rep").map(|s| s.as_str());

            if !apply_matches.contains_id("rules") {
                println!("Rules file must be supplied!");
                return;
            }
            let rules = apply_matches.get_one::<String>("rules").unwrap();
            
            // output rep may or may not exist: we don't care
            let out_rep = apply_matches.get_one::<String>("out_rep").map(|s| s.as_str());

            //apply_rules_cli(input, in_rep, rules, out_rep);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
