use std::env;
use std::process;

mod cli;
mod constants;
mod git;
mod utils;

use constants::{APP_NAME, HELP_LINE, TOGGLE_PREV, YES_FLAG};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut yes_flag: bool = false;

    let args: Vec<String> = args
        .into_iter()
        .filter(|arg| {
            if arg == YES_FLAG {
                yes_flag = true;
            }

            arg != YES_FLAG
        })
        .collect();

    let first_arg = args.get(1).map(|s| s.as_str()).unwrap_or_else(|| "_");
    let second_arg = args.get(2).map(|s| s.as_str()).unwrap_or_else(|| "");

    if args.len() > 3 {
        eprintln!(
            "{}: Too many arguments provided.\n\n{}",
            APP_NAME, HELP_LINE
        );
        process::exit(1);
    }

    match first_arg {
        "save" => {
            if let Err(e) = cli::save(second_arg, yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "remove" | "delete" => {
            if let Err(e) = cli::remove(second_arg, yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "use" => {
            if let Err(e) = cli::use_profile(second_arg, yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        TOGGLE_PREV => {
            if let Err(e) = cli::use_profile(TOGGLE_PREV, yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "discard" => {
            if let Err(e) = cli::discard_files(yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "_" => {
            // no command
            if let Err(e) = cli::list() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "version" | "--version" | "-v" => {
            cli::version();
        }
        "help" | "--help" | "-h" => {
            cli::help();
        }
        _ => {
            // wrong command
            eprintln!("{}: Unrecognized command.\n\n{}", APP_NAME, HELP_LINE);
        }
    }
}
