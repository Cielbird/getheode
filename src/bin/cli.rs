// this is the cli tool to interact with the getheode library most directly

use std::env;
use std::fs;

use clap::{Arg, ArgAction, Command};

use getheode;
use getheode::phonological_rule::PhonologicalRule;
use getheode::segment_string::SegmentString;

fn apply_rules_cli(input_file: &str, input_repr_file: Option<&str>, rules_file: &str, rules_repr_file: Option<&str>) {

    let input_rep;
    if let Some(f) = input_repr_file {
        match fs::read_to_string(input_file) {
            Ok(x) =>  {
                for (i, segs_str) in x.split("\n").enumerate() {
                    if segs_str.trim() == "" {
                        continue;
                    }

                }
            },
            Err(_) => { println!("rules file {input_file} is not valid") }
        }
    } else {
        input_rep = Vec::new();
    }

    let mut inputs = Vec::new();
    match fs::read_to_string(input_file) {
        Ok(x) =>  {
            for (i, segs_str) in x.split("\n").enumerate() {
                if segs_str.trim() == "" {
                    continue;
                }
                if let Some(f) = input_repr_file {
                    let representation = Representation::new()
                    match SegmentString::from_repr(segs_str.trim(), representation) {
                        Ok(string) => inputs.push(string),
                        Err(e) => println!("invalid input at line {}: {}",i+1,e),
                    }
                } else {
                    match SegmentString::new_worded(segs_str.trim()) {
                        Ok(string) => inputs.push(string),
                        Err(e) => println!("invalid input at line {}: {}",i+1,e),
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
    for input in &inputs {
        let mut output = input.clone();
        for rule in &rules {
            match rule.apply(&output) {
                Ok(out) => output = out,
                Err(e) => println!("rule apply failed: {}",e)
            }
        }
        println!("{}", output);
    }
}


fn cli() -> Command {
    Command::new("getheode")
        .about("getheode command line tool")
        .version("0.1.0") // TODO link to actual version
        .subcommand_required(true)
        .arg_required_else_help(true)
        // new subcommand
        // 
        .subcommand(
            Command::new("apply")
                //.short_flag('a')
                //.long_flag("apply")
                .about("apply phonological rules to words or other segment strings")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .help("a file of segment strings")
                        .action(ArgAction::Append)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("rules")
                        .long("rules")
                        .short('r')
                        .help("a file with phonological rules")
                        .action(ArgAction::Append)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .short('o')
                        .help("an output file")
                        .action(ArgAction::Append)
                        .num_args(1..),
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
            if !apply_matches.contains_id("rules") {
                println!("Rules file must be supplied!");
                return;
            }
            let rules = apply_matches.get_one::<String>("rules").unwrap();
            apply_rules_cli(input, rules);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
