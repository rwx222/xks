use std::env;
use std::process;

mod cli;
mod constants;
mod git;
mod utils;

use constants::{HELP_LINE, YES_FLAG};

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
        eprintln!("Expected at most two arguments, but more were provided.");
        println!("{}", HELP_LINE);
        process::exit(1);
    }

    match first_arg {
        "version" | "--version" | "-v" => {
            cli::version();
        }
        "save" => {
            if let Err(e) = cli::save(second_arg, yes_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "remove" => {
            if let Err(e) = cli::remove(second_arg, yes_flag) {
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
        _ => {
            // wrong command
            eprintln!("Unrecognized command. Please check the available commands.");
            println!("{}", HELP_LINE);
        }
    }
}
