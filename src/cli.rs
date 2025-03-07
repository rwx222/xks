use std::fs;
use std::io::ErrorKind;

use crate::constants::{
    GITCONFIG_FILE_NAME, PROFILE_NAME_MAX_LENGTH, REMOVING_DIR_ERR, USAGE_LINE, VALID_LINE, VERSION,
};
use crate::git;
use crate::utils;

pub fn list() -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    let mut current_profile_name = String::new();

    println!("[saved profiles: {}]", profile_dirs.len());

    for profile_directory in profile_dirs {
        let mut prefix: &str = " ";
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(&profile_directory),
        )?;

        if currfiles_prohash.hash == profile_prohash.hash {
            prefix = "*";
            current_profile_name = profile_directory.clone();
        }

        println!("{} {}", prefix, profile_directory);
    }
    println!("");

    if currfiles_prohash.tracked_file_names.len() == 0 {
        println!("--- no profile in use ---");
        println!("--- .gitconfig and SSH keys not found ---");
    } else if current_profile_name.is_empty() {
        println!("--- no profile in use ---");
        println!("--- current files have not been saved, or have been modified ---");
    } else {
        println!("(current: {})", current_profile_name);
    }

    if gitconfig_data.file_exists {
        println!("  gitconfig_name: {:?}", gitconfig_data.name);
        println!("  gitconfig_mail: {:?}", gitconfig_data.email);
    }

    println!(
        "  files ({:?}):",
        currfiles_prohash.tracked_file_names.len()
    );
    for filename in currfiles_prohash.tracked_file_names {
        println!("    {}", filename);
    }

    Ok(())
}

pub fn save(profile_name: &str, yes_flag: bool) -> Result<(), String> {
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
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);
    let profile_path = app_paths.data_dir_path.join(profile_name);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    if currfiles_prohash.tracked_file_names.len() == 0 {
        return Err("No files found to save for this profile.".to_string());
    }

    let profile_dirs: Vec<String> =
        utils::get_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let mut profile_already_exists_and_has_changes: bool = false;

    for profile_directory in profile_dirs {
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(&profile_directory),
        )?;

        if profile_directory == profile_name && currfiles_prohash.hash != profile_prohash.hash {
            profile_already_exists_and_has_changes = true;
        }
    }

    let save_profile = || -> Result<(), String> {
        if let Err(err) = fs::remove_dir_all(&profile_path) {
            if err.kind() != ErrorKind::NotFound {
                eprintln!("{}", REMOVING_DIR_ERR);
                return Err(err.to_string());
            }
        }

        if gitconfig_data.file_exists {
            if let Err(_) = utils::copy_file(
                &app_paths.gitconfig_file_path,
                &profile_path.join(GITCONFIG_FILE_NAME),
            ) {
                return Err(format!(
                    "Error: Could not copy file: {:?}",
                    GITCONFIG_FILE_NAME
                ));
            } else {
                println!("  {}", GITCONFIG_FILE_NAME)
            }
        }

        for filename in currfiles_prohash.tracked_file_names {
            if filename != GITCONFIG_FILE_NAME {
                if let Err(_) = utils::copy_file(
                    &app_paths.ssh_dir_path.join(&filename),
                    &profile_path.join(&filename),
                ) {
                    return Err(format!("Error: Could not copy file: {:?}", filename));
                } else {
                    println!("  {}", filename)
                }
            }
        }

        println!("Saved profile {:?} successfully!", profile_name);
        return Ok(());
    };

    if profile_already_exists_and_has_changes && !yes_flag {
        let prompt = "The current files have been modified.\nDo you want to save the changes?";

        if utils::confirm(prompt) {
            return save_profile();
        } else {
            println!("No profile was saved!");
            return Ok(());
        }
    }

    save_profile()
}

pub fn remove(profile_name: &str, yes_flag: bool) -> Result<(), String> {
    if profile_name.is_empty() {
        let lines = [
            "Profile name cannot be empty.",
            "Example: xks remove alex_github",
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let remove_profile = || -> Result<(), String> {
        let app_paths = utils::get_app_paths();
        let profile_path = app_paths.data_dir_path.join(profile_name);

        if let Err(err) = fs::remove_dir_all(&profile_path) {
            if err.kind() != ErrorKind::NotFound {
                eprintln!("{}", REMOVING_DIR_ERR);
                return Err(err.to_string());
            } else {
                println!("Profile not found. Nothing to remove.");
                return Ok(());
            }
        }

        println!("Removed profile {:?} successfully!", profile_name);
        return Ok(());
    };

    if yes_flag {
        return remove_profile();
    }

    let prompt = "Are you sure you want to remove this profile?";

    if utils::confirm(prompt) {
        return remove_profile();
    } else {
        println!("No profile was removed!");
        return Ok(());
    }
}

pub fn discard_files(yes_flag: bool) -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    if currfiles_prohash.tracked_file_names.len() == 0 {
        println!(".gitconfig and SSH keys not found.");
        println!("No files were discarded!");
        return Ok(());
    }

    let mut is_profile_saved: bool = false;

    for profile_directory in profile_dirs {
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(&profile_directory),
        )?;

        if currfiles_prohash.hash == profile_prohash.hash {
            is_profile_saved = true;
        }
    }

    let remove_current_files = || -> Result<(), String> {
        for filename in &currfiles_prohash.tracked_file_names {
            let file_to_remove_path = if filename == GITCONFIG_FILE_NAME {
                app_paths.gitconfig_file_path.clone()
            } else {
                app_paths.ssh_dir_path.join(filename)
            };

            if let Err(err) = fs::remove_file(file_to_remove_path) {
                eprintln!("Error: Could not remove file: {:?}", filename);
                return Err(err.to_string());
            }
        }

        println!("Current files discarded successfully!");
        return Ok(());
    };

    if is_profile_saved || yes_flag {
        return remove_current_files();
    }

    println!(
        "Current files ({:?}):",
        currfiles_prohash.tracked_file_names.len()
    );
    for filename in &currfiles_prohash.tracked_file_names {
        println!("  {}", filename);
    }

    let prompt = "The current files have not been saved, or have been modified.\nThis action will remove them.\nAre you sure you want to discard them?";

    if utils::confirm(prompt) {
        return remove_current_files();
    } else {
        println!("No files were discarded!");
        return Ok(());
    }
}

pub fn use_profile(new_profile_name: &str, yes_flag: bool) -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    let mut current_profile_name = String::new();
    let mut is_profile_saved: bool = false;
    let mut profile_name_exists: bool = false;

    for profile_directory in profile_dirs {
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(&profile_directory),
        )?;

        if currfiles_prohash.hash == profile_prohash.hash {
            is_profile_saved = true;
            current_profile_name = profile_directory.clone();
        }
        if profile_directory == new_profile_name {
            profile_name_exists = true;
        }
    }

    if !profile_name_exists {
        println!("Profile {:?} not found.", new_profile_name);

        if current_profile_name.is_empty() {
            println!("No profile in use.");
        } else {
            println!("Current: {:?}", current_profile_name);
        }
        return Ok(());
    }

    let change_profile = || -> Result<(), String> {
        for filename in &currfiles_prohash.tracked_file_names {
            let file_to_remove_path = if filename == GITCONFIG_FILE_NAME {
                app_paths.gitconfig_file_path.clone()
            } else {
                app_paths.ssh_dir_path.join(filename)
            };

            if let Err(err) = fs::remove_file(file_to_remove_path) {
                eprintln!("Error: Could not remove file: {:?}", filename);
                return Err(err.to_string());
            }
        }

        let new_profile_source_path = app_paths.data_dir_path.join(new_profile_name);
        let new_profile_file_names: Vec<String> =
            utils::get_files(&new_profile_source_path).unwrap_or_else(|_| vec![]);

        for filename in new_profile_file_names {
            let destination_file_path = if filename == GITCONFIG_FILE_NAME {
                &app_paths.gitconfig_file_path
            } else {
                &app_paths.ssh_dir_path.join(&filename)
            };

            if let Err(_) = utils::copy_file(
                &new_profile_source_path.join(&filename),
                destination_file_path,
            ) {
                return Err(format!("Error: Could not copy file: {:?}", filename));
            } else {
                println!("  {}", filename)
            }
        }

        println!("Profile switched successfully!");
        println!("Now using profile: {:?}", new_profile_name);
        return Ok(());
    };

    if is_profile_saved || yes_flag {
        return change_profile();
    };

    let prompt = "The current files have not been saved, or have been modified.\nThis action will remove them.\nAre you sure you want to continue?";

    if utils::confirm(prompt) {
        return change_profile();
    } else {
        println!("Profile switch canceled.");
        if current_profile_name.is_empty() {
            println!("No profile in use.");
        } else {
            println!("Current: {:?}", current_profile_name);
        }
        return Ok(());
    }
}

pub fn version() {
    println!("{}", VERSION);
}
