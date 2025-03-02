use std::fs;
use std::path::PathBuf;

pub fn get_profile_dirs(path: &PathBuf) -> Vec<String> {
    let mut profile_dirs: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.filter_map(Result::ok) {
            if entry.path().is_dir() {
                profile_dirs.push(entry.file_name().to_string_lossy().into_owned())
            }
        }
    }

    profile_dirs
}
