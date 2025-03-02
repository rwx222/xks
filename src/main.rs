use std::env;
use std::path::Path;

mod config;
mod git;

fn main() {
    const DATA_DIR_NAME: &str = ".xks";
    const GITCONFIG_FILE: &str = ".gitconfig";

    let args: Vec<String> = env::args().collect();

    let home_path: String = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));

    let data_path = Path::new(&home_path).join(DATA_DIR_NAME);
    let gitconfig_path = Path::new(&home_path).join(GITCONFIG_FILE);

    let profile_dirs = config::get_profile_dirs(&data_path);
    let gitconfig_data = git::get_gitconfig_data(&gitconfig_path);

    println!("{:?}", args);
    println!("{:?}", profile_dirs);
    println!("{:?}", data_path);
    println!("{:?}", gitconfig_data.name);
    println!("{:?}", gitconfig_data.email);
}
