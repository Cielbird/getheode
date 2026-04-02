// this is the cli tool to interact with the getheode library most directly

use std::fs::{self, read_to_string};
use std::path::Path;

use clap::{Arg, Command};

use getheode::GETHEODE_VERSION;

/// Returns the contents of a file if the input is a valid file path.
/// Otherwise, returns the input string itself.
pub fn file_or_raw(input: &str) -> Result<String, String> {
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
            Command::new("surface")
                .about("gets the surface representation of the words in a lect")
                .arg(
                    Arg::new("phoneme_bank")
                        .short('b')
                        .value_name("PHONEMES_FILE")
                        .required(true)
                        .help("a file with a phoneme bank"),
                )
                .arg(
                    Arg::new("rules")
                        .short('r')
                        .value_name("RULES_FILE")
                        .required(true)
                        .help("a file with realization rules"),
                )
                .arg(
                    Arg::new("phonotactics")
                        .short('p')
                        .value_name("PHONOTACTICS_FILE")
                        .required(false)
                        .help("a file with phonotactics"),
                )
                .arg(
                    Arg::new("input")
                        .short('i')
                        .value_name("INPUT")
                        .required(true)
                        .help("file containing phonemes to parse. if not provided, will take inout from stdin."),
                ),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("surface", arg_matches)) => {
            let bank = arg_matches.get_one::<String>("phoneme_bank").unwrap();
            let _bank = read_to_string(bank).unwrap();

            let rules = arg_matches.get_one::<String>("rules").unwrap();
            let _rules = read_to_string(rules).unwrap();

            let _phonotactics = arg_matches
                .get_one::<String>("phonotactics")
                .map(|s| read_to_string(s).unwrap().clone());

            let input_file = arg_matches.get_one::<String>("input").unwrap();
            let _input = read_to_string(Path::new(&input_file)).unwrap();

            unimplemented!("Major refactor, this is unimplemented");
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
