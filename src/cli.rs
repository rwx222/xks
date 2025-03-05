use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::constants::{
    GITCONFIG_FILE, PROFILE_NAME_MAX_LENGTH, READING_FILES_ERR, REMOVING_DIR_ERR, SSH_FILES,
    USAGE_LINE, VALID_LINE, VERSION,
};
use crate::git;
use crate::utils;

pub fn list() -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file);
    let profile_dirs: Vec<String> = utils::get_dirs(&app_paths.data_dir).unwrap_or_else(|_| vec![]);

    let all_ssh_files: Vec<String> =
        utils::get_files(&app_paths.ssh_dir).unwrap_or_else(|_| vec![]);
    let mut ssh_files: Vec<String> = all_ssh_files
        .into_iter()
        .filter(|filename| SSH_FILES.contains(&filename.as_str()))
        .collect();

    if gitconfig_data.file_exists {
        ssh_files.push(GITCONFIG_FILE.to_string());
    }

    ssh_files.sort();

    let mut current_profile_sum_paths: Vec<PathBuf> = vec![];

    ssh_files.iter().for_each(|filename| {
        if filename.as_str() == GITCONFIG_FILE {
            current_profile_sum_paths.push(app_paths.gitconfig_file.clone());
        } else {
            current_profile_sum_paths.push(app_paths.ssh_dir.join(&filename));
        }
    });

    let current_hash =
        utils::get_profile_hash(&current_profile_sum_paths).unwrap_or_else(|_| String::from("---"));
    let mut current_profile_name = String::new();

    println!("[saved profiles: {}]", profile_dirs.len());

    for profile_dir in profile_dirs {
        let mut prefix: &str = " ";
        let profile_path = app_paths.data_dir.join(&profile_dir);
        let all_profile_files: Vec<String> = utils::get_files(&profile_path)?;
        let mut filtered_files: Vec<String> = all_profile_files
            .into_iter()
            .filter(|filename| {
                SSH_FILES.contains(&filename.as_str()) || filename.as_str() == GITCONFIG_FILE
            })
            .collect();
        filtered_files.sort();

        let mut profile_sum_paths: Vec<PathBuf> = vec![];

        filtered_files
            .into_iter()
            .for_each(|filename| profile_sum_paths.push(profile_path.join(filename)));

        let profile_hash = utils::get_profile_hash(&profile_sum_paths).expect(READING_FILES_ERR);

        if profile_hash == current_hash {
            prefix = "*";
            current_profile_name = profile_dir.clone();
        }

        println!("{} {}", prefix, profile_dir);
    }
    println!("");

    if current_profile_name.is_empty() {
        println!("--- no profile has been saved for the current files ---");
    } else {
        println!("(current = {})", current_profile_name);
    }

    if gitconfig_data.file_exists {
        println!("  gitconfig_name: {:?}", gitconfig_data.name);
        println!("  gitconfig_mail: {:?}", gitconfig_data.email);
    }

    println!("  files ({:?}):", ssh_files.len());
    for filename in ssh_files {
        println!("    {}", filename);
    }

    Ok(())
}

pub fn save(profile_name: &str) -> Result<(), String> {
    let valid_chars = |c: char| c.is_ascii_alphanumeric() || "@-_.".contains(c);

    if profile_name.is_empty() {
        let lines = ["Profile name cannot be empty.", USAGE_LINE, VALID_LINE];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if !profile_name.starts_with(|c: char| c.is_ascii_alphanumeric())
        || !profile_name.ends_with(|c: char| c.is_ascii_alphanumeric())
    {
        let lines = [
            format!("Invalid name {:?}", profile_name),
            "Profile name must start and end with a letter or number: AZaz09".to_string(),
            USAGE_LINE.to_string(),
            VALID_LINE.to_string(),
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if !profile_name.chars().all(valid_chars) {
        let lines = [
            format!("Invalid name {:?}", profile_name),
            "Profile name can only contain letters and: @-_.".to_string(),
            USAGE_LINE.to_string(),
            VALID_LINE.to_string(),
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if profile_name.chars().count() > PROFILE_NAME_MAX_LENGTH {
        let lines = [format!(
            "Profile name cannot exceed {} characters.",
            PROFILE_NAME_MAX_LENGTH
        )];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file);
    let profile_path = app_paths.data_dir.join(profile_name);

    let all_ssh_files: Vec<String> =
        utils::get_files(&app_paths.ssh_dir).unwrap_or_else(|_| vec![]);
    let ssh_files: Vec<String> = all_ssh_files
        .into_iter()
        .filter(|filename| SSH_FILES.contains(&filename.as_str()))
        .collect();

    if !gitconfig_data.file_exists && ssh_files.len() == 0 {
        return Err("No files found to save for this profile.".to_string());
    }

    if let Err(err) = fs::remove_dir_all(&profile_path) {
        if err.kind() != ErrorKind::NotFound {
            return Err(REMOVING_DIR_ERR.to_string());
        }
    }

    if gitconfig_data.file_exists {
        if let Err(_) = utils::copy_file(
            &app_paths.gitconfig_file,
            &profile_path.join(GITCONFIG_FILE),
        ) {
            return Err(format!("Error: Could not copy file: {:?}", GITCONFIG_FILE));
        } else {
            println!("File copied!: {}", GITCONFIG_FILE)
        }
    }

    for ssh_file in &ssh_files {
        if let Err(_) = utils::copy_file(
            &app_paths.ssh_dir.join(ssh_file),
            &profile_path.join(ssh_file),
        ) {
            return Err(format!("Error: Could not copy file: {:?}", ssh_file));
        } else {
            println!("File copied!: {}", ssh_file)
        }
    }

    println!("Saved profile {:?} successfully!", profile_name);
    Ok(())
}

pub fn remove(profile_name: &str) -> Result<(), String> {
    if profile_name.is_empty() {
        let lines = [
            "Profile name cannot be empty.",
            "Example: xks remove alex_github",
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let app_paths = utils::get_app_paths();
    let profile_path = app_paths.data_dir.join(profile_name);

    if let Err(err) = fs::remove_dir_all(&profile_path) {
        if err.kind() != ErrorKind::NotFound {
            return Err(REMOVING_DIR_ERR.to_string());
        } else {
            println!("Profile not found. Nothing to remove.");
            return Ok(());
        }
    }

    println!("Removed profile {:?} successfully!", profile_name);
    Ok(())
}

pub fn version() {
    println!("xks v{}", VERSION);
}
