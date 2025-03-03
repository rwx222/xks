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

    match first_arg {
        "version" | "--version" => {
            cli::version();
        }
        "save" => {
            if let Err(e) = cli::save(second_arg) {
                eprintln!("{}", e);
                process::exit(1);
            } else {
                if let Err(e) = cli::list() {
                    eprintln!("{}", e);
                    process::exit(1);
                }
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
            if let Err(e) = cli::list() {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
    }
}
