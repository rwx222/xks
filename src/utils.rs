use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::constants::{DATA_DIR_NAME, GITCONFIG_FILE, READING_DIR_ERR, SSH_DIR};

pub struct AppPaths {
    pub gitconfig_file: PathBuf,
    pub data_dir: PathBuf,
    pub ssh_dir: PathBuf,
}

pub fn get_app_paths() -> AppPaths {
    let home_path: String = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));

    let gitconfig_file = Path::new(&home_path).join(GITCONFIG_FILE);
    let data_dir = Path::new(&home_path).join(DATA_DIR_NAME);
    let ssh_dir = Path::new(&home_path).join(SSH_DIR);

    AppPaths {
        gitconfig_file,
        data_dir,
        ssh_dir,
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

pub fn get_profile_hash(file_paths: &Vec<PathBuf>) -> io::Result<String> {
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
