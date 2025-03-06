use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use crate::constants::{
    DATA_DIR_NAME, GITCONFIG_FILE_NAME, READING_DIR_ERR, READING_HASH_FILES_ERR, SSH_DIR,
    TRACKED_FILE_NAMES,
};

pub struct AppPaths {
    pub gitconfig_file_path: PathBuf,
    pub data_dir_path: PathBuf,
    pub ssh_dir_path: PathBuf,
}

pub fn get_app_paths() -> AppPaths {
    let home_path: String = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));

    let gitconfig_file_path = Path::new(&home_path).join(GITCONFIG_FILE_NAME);
    let data_dir_path = Path::new(&home_path).join(DATA_DIR_NAME);
    let ssh_dir_path = Path::new(&home_path).join(SSH_DIR);

    AppPaths {
        gitconfig_file_path,
        data_dir_path,
        ssh_dir_path,
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

pub fn get_dirs<T: AsRef<Path>>(path: T) -> Result<Vec<String>, String> {
    let mut data: Vec<String> = Vec::new();

    let entries = fs::read_dir(&path).map_err(|e| format!("{}\n{}", READING_DIR_ERR, e))?;

    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_dir() {
            data.push(entry.file_name().to_string_lossy().into_owned())
        }
    }

    Ok(data)
}

pub fn get_files<T: AsRef<Path>>(path: T) -> Result<Vec<String>, String> {
    let mut data: Vec<String> = Vec::new();

    let entries = fs::read_dir(&path).map_err(|e| format!("{}\n{}", READING_DIR_ERR, e))?;

    for entry in entries.filter_map(Result::ok) {
        if entry.path().is_file() {
            data.push(entry.file_name().to_string_lossy().into_owned())
        }
    }

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

pub fn copy_file(from_path: &PathBuf, to_path: &PathBuf) -> io::Result<()> {
    if let Some(parent) = to_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(from_path, to_path)?;

    Ok(())
}

pub fn confirm(prompt: &str) -> bool {
    print!("{} [yes/no] (y/n): ", prompt);
    io::stdout().flush().expect("Error: Writing stdout.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: Reading input.");

    matches!(input.trim().to_lowercase().as_str(), "yes" | "y")
}
