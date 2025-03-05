use std::env;
use std::process;

mod cli;
mod constants;
mod git;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = args.get(1).map(|s| s.as_str()).unwrap_or_else(|| "_");
    let second_arg = args.get(2).map(|s| s.as_str()).unwrap_or_else(|| "");

    if args.len() > 3 {
        eprintln!("Expected at most two arguments, but more were provided.");
        println!("{}", constants::HELP_LINE);
        process::exit(1);
    }

    match first_arg {
        "version" | "--version" => {
            cli::version();
        }
        "save" => {
            if let Err(e) = cli::save(second_arg) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        "remove" | "delete" => {
            if let Err(e) = cli::remove(second_arg) {
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
            println!("{}", constants::HELP_LINE);
        }
    }
}
