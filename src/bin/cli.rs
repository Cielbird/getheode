// this is the cli tool to interact with the getheode library most directly

use std::fs;
use std::path::Path;

use clap::{Arg, Command};

use getheode;
use getheode::lect::Lect;
use getheode::GETHEODE_VERSION;
use regex::Regex;

/// Returns the contents of a file if the input is a valid file path.
/// Otherwise, returns the input string itself.
pub fn file_or_raw(input: &str) -> Result<String, String> {
    if Path::new(input).exists() {
        fs::read_to_string(input)
            .map_err(|e| format!("Failed to read file '{}': {}", input, e))
    } else {
        Ok(input.to_string())
    }
}

fn read_lect_file(lect_path: &str) -> Result<Lect, String>{
    return Lect::from_yaml(&lect_path).map_err(|e| e.to_string());
}

fn valid (lect: &str, input: &str) {
    // validate parameters etc.
    let lect: Lect = read_lect_file(lect).expect("failed reading lect file");
    let input: String = file_or_raw(input).expect("failed reading input");

    // regex to match text within slashes (/likethis/)
    let re = Regex::new(r"/([^/]+)/").expect("Invalid regex");
    
    // Iterate over the matches and output validity
    re.captures_iter(&input)
        .filter_map(|cap| cap.get(1)) // Get the first capture group (text inside slashes)
        .map(|m| m.as_str()) // Extract the matched string slice
        .all(|s| {
            let valid = lect.validate_word(s);
            println!("/{}/: {}", s, if valid { "valid" } else { "invalid" });
            valid
        });
}

fn surface (lect: &str, input: &str) {
    println!("getting surface rep!");
    unimplemented!();
}

fn gen (lect: &str, count: u32, surface: bool) {
    println!("generating words!");
    unimplemented!();
}

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
                        .value_parser(clap::value_parser!(u32).range(0..=99)),
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
        Some(("valid", arg_matches)) => {
            let lect = arg_matches.get_one::<String>("lect").unwrap();
            let input = arg_matches.get_one::<String>("input").unwrap();
            
            valid(lect, input);
        }
        Some(("surface", arg_matches)) => {
            let lect = arg_matches.get_one::<String>("lect").unwrap();
            let input = arg_matches.get_one::<String>("input").unwrap();
            
            surface(lect, input);
        }
        Some(("gen", arg_matches)) => {
            let lect = arg_matches.get_one::<String>("lect").unwrap();
            let count = arg_matches.get_one::<u32>("count").unwrap();
            let surface = matches.get_flag("surface");
            
            gen(lect, *count, surface);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
