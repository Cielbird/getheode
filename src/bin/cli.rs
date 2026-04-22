// this is the cli tool to interact with the getheode library most directly

use std::{fs, path::Path};

use clap::{Arg, Command};

use getheode::{
    GETHEODE_VERSION,
    phonology::{rule::PhonoRuleSet, string::PhonoString},
};

fn file_or_raw(input: &str) -> Result<String, String> {
    if Path::new(input).exists() {
        fs::read_to_string(input).map_err(|e| format!("Failed to read file '{}': {}", input, e))
    } else {
        Ok(input.to_string())
    }
}

fn cli() -> Command {
    Command::new("getheode")
        .about("getheode command line tool")
        .version(GETHEODE_VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("apply")
                .about("apply a phonological rule to a phonological string")
                .arg(
                    Arg::new("rule")
                        .short('r')
                        .value_name("RULE")
                        .required(true)
                        .help("rule string or file"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .value_name("INPUT")
                        .required(true)
                        .help("phonological string or file"),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("apply", args)) => {
            let rule_str =
                file_or_raw(args.get_one::<String>("rule").unwrap()).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    std::process::exit(1);
                });
            let input_str =
                file_or_raw(args.get_one::<String>("input").unwrap()).unwrap_or_else(|e| {
                    eprintln!("{e}");
                    std::process::exit(1);
                });

            let opts = Default::default();
            let rule_set = PhonoRuleSet::parse(rule_str.trim(), opts).unwrap_or_else(|e| {
                eprintln!("Error parsing rule \"{rule_str}\"\n{e}");
                std::process::exit(1);
            });
            let (_, string) = PhonoString::parse(input_str.trim()).unwrap_or_else(|e| {
                eprintln!("Error parsing input \"{input_str}\"\n{e}");
                std::process::exit(1);
            });

            println!("{}", rule_set.apply(string));
        }
        _ => unreachable!(),
    }
}
