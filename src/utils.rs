use sha2::{Digest, Sha256};
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};

use crate::constants::{
    CONFIG_DIR_NAME, DATA_DIR_NAME, GITCONFIG_FILE_NAME, PREVIOUS_PROFILE_FILE_NAME,
    READING_DIR_ERR, READING_HASH_FILES_ERR, SSH_DIR, TRACKED_FILE_NAMES,
};

pub struct AppPaths {
    pub gitconfig_file_path: PathBuf,
    pub data_dir_path: PathBuf,
    pub ssh_dir_path: PathBuf,
    pub previous_profile_file_path: PathBuf,
}

pub fn get_app_paths() -> AppPaths {
    let home_path: String = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));

    let gitconfig_file_path = Path::new(&home_path).join(GITCONFIG_FILE_NAME);
    let data_dir_path = Path::new(&home_path).join(DATA_DIR_NAME);
    let ssh_dir_path = Path::new(&home_path).join(SSH_DIR);
    let previous_profile_file_path = Path::new(&data_dir_path)
        .join(CONFIG_DIR_NAME)
        .join(PREVIOUS_PROFILE_FILE_NAME);

    AppPaths {
        gitconfig_file_path,
        data_dir_path,
        ssh_dir_path,
        previous_profile_file_path,
    }
}

#[derive(Debug)]
pub struct ProHash {
    pub hash: String,
    pub tracked_file_names: Vec<String>,
}

pub fn get_profile_hash(
    app_paths: &AppPaths,
    current_gitconfig_exists: bool,
    profile_name: Option<&String>,
) -> Result<ProHash, String> {
    match profile_name {
        Some(profile_dir) => {
            let profile_path = app_paths.data_dir_path.join(profile_dir);

            let all_file_names: Vec<String> = get_files(&profile_path)?;
            let mut tracked_file_names: Vec<String> = all_file_names
                .into_iter()
                .filter(|filename| TRACKED_FILE_NAMES.contains(&filename.as_str()))
                .collect();

            tracked_file_names.sort();

            let mut sum_paths: Vec<PathBuf> = vec![];

            tracked_file_names
                .iter()
                .for_each(|filename| sum_paths.push(profile_path.join(filename)));

            let profile_hash = get_files_hash(sum_paths).unwrap_or_else(|_| String::from(""));

            if profile_hash.is_empty() {
                return Err(READING_HASH_FILES_ERR.to_string());
            }

            return Ok(ProHash {
                hash: profile_hash,
                tracked_file_names,
            });
        }
        None => {
            let all_file_names: Vec<String> =
                get_files(&app_paths.ssh_dir_path).unwrap_or_else(|_| vec![]);
            let mut tracked_file_names: Vec<String> = all_file_names
                .into_iter()
                .filter(|filename| {
                    TRACKED_FILE_NAMES.contains(&filename.as_str())
                        && filename != GITCONFIG_FILE_NAME
                })
                .collect();

            if current_gitconfig_exists {
                tracked_file_names.push(GITCONFIG_FILE_NAME.to_string());
            }

            tracked_file_names.sort();

            let mut sum_paths: Vec<PathBuf> = vec![];

            tracked_file_names.iter().for_each(|filename| {
                if filename == GITCONFIG_FILE_NAME {
                    sum_paths.push(app_paths.gitconfig_file_path.clone());
                } else {
                    sum_paths.push(app_paths.ssh_dir_path.join(filename))
                }
            });

            let profile_hash = get_files_hash(sum_paths).unwrap_or_else(|_| String::from(""));

            if profile_hash.is_empty() {
                return Err(READING_HASH_FILES_ERR.to_string());
            }

            return Ok(ProHash {
                hash: profile_hash,
                tracked_file_names,
            });
        }
    }
}

pub fn get_profile_dirs<T: AsRef<Path>>(path: T) -> Result<Vec<String>, String> {
    let mut data: Vec<String> = Vec::new();

    let entries = fs::read_dir(&path).map_err(|e| format!("{}\n\n{}", READING_DIR_ERR, e))?;

    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_dir() {
            let item_name = entry.file_name().to_string_lossy().into_owned();

            if !item_name.starts_with(".") && !item_name.contains(" ") {
                data.push(item_name);
            }
        }
    }

    data.sort();

    Ok(data)
}

pub fn get_files<T: AsRef<Path>>(path: T) -> Result<Vec<String>, String> {
    let mut data: Vec<String> = Vec::new();

    let entries = fs::read_dir(&path).map_err(|e| format!("{}\n\n{}", READING_DIR_ERR, e))?;

    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_file() {
            let item_name = entry.file_name().to_string_lossy().into_owned();
            data.push(item_name);
        }
    }

    data.sort();

    Ok(data)
}

fn hash_file<T: AsRef<Path>>(path: T, hasher: &mut Sha256) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0; 1024];
    while let Ok(n) = file.read(&mut buffer) {
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(())
}

pub fn get_files_hash(file_paths: Vec<PathBuf>) -> io::Result<String> {
    let mut hasher = Sha256::new();
    for file_path in file_paths {
        hash_file(file_path, &mut hasher)?;
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn copy_file(source_file_path: &PathBuf, destination_file_path: &PathBuf) -> io::Result<()> {
    if let Some(parent) = destination_file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source_file_path, destination_file_path)?;

    Ok(())
}

pub fn confirm(prompt: &str) -> bool {
    print!("\n{} [yes/no] (y/n): ", prompt);
    io::stdout().flush().expect("Error: Writing stdout.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Reading input.");

    matches!(input.trim().to_lowercase().as_str(), "yes" | "y")
}

pub fn read_first_line<T: AsRef<Path>>(file_path: T) -> String {
    let res = match File::open(&file_path) {
        Ok(file) => {
            let mut lines = io::BufReader::new(file).lines();

            lines
                .next()
                .unwrap_or_else(|| Ok(String::new()))
                .map(|line| line.trim().to_string())
                .unwrap_or_default()
        }
        Err(_) => String::new(),
    };

    res
}

pub fn write_to_file(file_path: PathBuf, content: &str) -> io::Result<()> {
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;

    Ok(())
}

pub fn get_new_use_profile_name(
    app_paths: &AppPaths,
    profile_dirs: &Vec<String>,
    current_profile_name: &String,
    profile_name: &str,
) -> String {
    if profile_name == "-" && profile_dirs.len() > 0 {
        if profile_dirs.len() == 1 {
            return profile_dirs[0].clone();
        } else if profile_dirs.len() == 2 {
            if let Some(first_different_profile) = profile_dirs
                .iter()
                .find(|&item| item != current_profile_name)
            {
                return first_different_profile.clone();
            }
        } else {
            let previous_profile_name = read_first_line(&app_paths.previous_profile_file_path);

            if !previous_profile_name.is_empty() {
                let previous_profile_source_path =
                    app_paths.data_dir_path.join(&previous_profile_name);
                let previous_profile_exists: bool =
                    previous_profile_source_path.exists() && previous_profile_source_path.is_dir();

                if previous_profile_exists {
                    return previous_profile_name;
                }
            }

            if let Some(first_different_profile) = profile_dirs
                .iter()
                .find(|&item| item != current_profile_name)
            {
                return first_different_profile.clone();
            }
        }
    }

    profile_name.to_string()
}
