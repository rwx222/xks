use std::fs;
use std::io::ErrorKind;

use crate::constants::{
    APP_NAME, GITCONFIG_FILE_NAME, PROFILE_NAME_MAX_LENGTH, REMOVING_DIR_ERR, TOGGLE_PREV, VERSION,
};
use crate::git;
use crate::utils;

pub fn save(profile_name: &str, yes_flag: bool) -> Result<(), String> {
    let valid_save_examples: String = format!(
        "Examples:\n    {} save alex\n    {} save alex_2@wi-fi.org",
        APP_NAME, APP_NAME
    );
    let valid_chars = |c: char| c.is_ascii_alphanumeric() || "@-_.".contains(c);

    if profile_name.is_empty() {
        let lines = [
            format!("{}: Profile name cannot be empty.\n", APP_NAME),
            valid_save_examples,
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if !profile_name.starts_with(|c: char| c.is_ascii_alphanumeric())
        || !profile_name.ends_with(|c: char| c.is_ascii_alphanumeric())
    {
        let lines = [
            format!("{}: Invalid profile name {:?}\n", APP_NAME, profile_name),
            "Profile names must start and end with a letter or number.\n".to_string(),
            valid_save_examples,
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if !profile_name.chars().all(valid_chars) {
        let lines = [
            format!("{}: Invalid profile name {:?}\n", APP_NAME, profile_name),
            "Profile names can only contain: (letters, numbers, @, -, _, .)\n".to_string(),
            valid_save_examples,
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    if profile_name.chars().count() > PROFILE_NAME_MAX_LENGTH {
        let lines = [
            format!(
                "{}: Profile name cannot exceed {} characters.\n",
                APP_NAME, PROFILE_NAME_MAX_LENGTH
            ),
            valid_save_examples,
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);
    let profile_path = app_paths.data_dir_path.join(profile_name);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    if currfiles_prohash.tracked_file_names.len() == 0 {
        return Err(format!(
            "{}: Current files not found.\n\nNo profile was saved.",
            APP_NAME
        ));
    }

    let profile_dirs: Vec<String> =
        utils::get_profile_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

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
                eprintln!("{}: {}\n", APP_NAME, REMOVING_DIR_ERR);
                return Err(err.to_string());
            }
        }

        if gitconfig_data.file_exists {
            if let Err(_) = utils::copy_file(
                &app_paths.gitconfig_file_path,
                &profile_path.join(GITCONFIG_FILE_NAME),
            ) {
                return Err(format!(
                    "{}: Error: Could not copy file: {}",
                    APP_NAME, GITCONFIG_FILE_NAME
                ));
            }
        }

        for filename in currfiles_prohash.tracked_file_names {
            if filename != GITCONFIG_FILE_NAME {
                if let Err(_) = utils::copy_file(
                    &app_paths.ssh_dir_path.join(&filename),
                    &profile_path.join(&filename),
                ) {
                    return Err(format!(
                        "{}: Error: Could not copy file: {}",
                        APP_NAME, filename
                    ));
                }
            }
        }

        println!("\nProfile {:?} saved successfully!", profile_name);
        return Ok(());
    };

    if profile_already_exists_and_has_changes && !yes_flag {
        let prompt = "The current files have been modified.\nDo you want to save the changes?";

        if utils::confirm(prompt) {
            return save_profile();
        } else {
            println!("\nNo profile was saved.");
            return Ok(());
        }
    }

    save_profile()
}

pub fn remove(profile_name: &str, yes_flag: bool) -> Result<(), String> {
    if profile_name.is_empty() {
        let lines = [
            format!("{}: Profile name cannot be empty.\n", APP_NAME),
            format!("Example:\n    {} remove alex", APP_NAME),
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let app_paths = utils::get_app_paths();
    let profile_path = app_paths.data_dir_path.join(profile_name);
    let profile_exists: bool = profile_path.exists() && profile_path.is_dir();

    let non_existing_profile_msg = format!(
        "{}: Profile {:?} not found.\n\nNo profile was removed.",
        APP_NAME, profile_name
    );

    if !profile_exists {
        return Err(non_existing_profile_msg);
    }

    let remove_profile = || -> Result<(), String> {
        if let Err(err) = fs::remove_dir_all(&profile_path) {
            if err.kind() == ErrorKind::NotFound {
                return Err(non_existing_profile_msg);
            } else {
                eprintln!("{}: {}\n", APP_NAME, REMOVING_DIR_ERR);
                return Err(err.to_string());
            }
        }

        println!("\nProfile {:?} removed successfully!", profile_name);
        return Ok(());
    };

    if yes_flag {
        return remove_profile();
    }

    let prompt = format!(
        "This action will delete the profile {:?} and all its files.\nAre you sure you want to proceed?",
        profile_name
    );

    if utils::confirm(prompt.as_str()) {
        return remove_profile();
    } else {
        println!("\nNo profile was removed.");
        return Ok(());
    }
}

pub fn use_profile(input_profile_name: &str, yes_flag: bool) -> Result<(), String> {
    if input_profile_name.is_empty() {
        let lines = [
            format!("{}: Profile name cannot be empty.\n", APP_NAME),
            format!("Example:\n    {} use alex", APP_NAME),
        ];
        let msg = lines.join("\n");
        return Err(msg);
    }

    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_profile_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    let mut current_profile_names: Vec<String> = vec![];
    let mut is_profile_saved: bool = false;

    for profile_directory in &profile_dirs {
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(profile_directory),
        )?;

        if currfiles_prohash.hash == profile_prohash.hash {
            is_profile_saved = true;
            current_profile_names.push(profile_directory.clone());
        }
    }

    let new_profile_name = utils::get_new_use_profile_name(
        &app_paths,
        &profile_dirs,
        &current_profile_names,
        input_profile_name,
    );
    let new_profile_source_path = app_paths.data_dir_path.join(&new_profile_name);
    let profile_exists: bool = new_profile_source_path.exists() && new_profile_source_path.is_dir();

    let profile_in_use_msg = if current_profile_names.is_empty() {
        String::from("No profile in use.")
    } else {
        format!("Profile in use: {:?}", current_profile_names.join(" - "))
    };

    if profile_dirs.is_empty() {
        return Err(format!(
            "{}: No saved profiles available to use.\n\n{}",
            APP_NAME, profile_in_use_msg
        ));
    } else if new_profile_name == TOGGLE_PREV {
        return Err(format!(
            "{}: No saved profiles different from the current one.\n\n{}",
            APP_NAME, profile_in_use_msg
        ));
    } else if !profile_exists {
        return Err(format!(
            "{}: Profile {:?} not found.\n\n{}",
            APP_NAME, new_profile_name, profile_in_use_msg
        ));
    }

    let change_profile = || -> Result<(), String> {
        for filename in &currfiles_prohash.tracked_file_names {
            let file_to_remove_path = if filename == GITCONFIG_FILE_NAME {
                app_paths.gitconfig_file_path.clone()
            } else {
                app_paths.ssh_dir_path.join(filename)
            };

            if let Err(err) = fs::remove_file(file_to_remove_path) {
                eprintln!("{}: Error: Could not remove file: {}\n", APP_NAME, filename);
                return Err(err.to_string());
            }
        }

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
                return Err(format!(
                    "{}: Error: Could not copy file: {}",
                    APP_NAME, filename
                ));
            }
        }

        if current_profile_names.len() > 0 {
            utils::write_to_file(
                app_paths.previous_profile_file_path,
                &current_profile_names[0],
            )
            .ok();
        }

        println!(
            "\nProfile switched successfully!\n\nUsing profile: {:?}",
            new_profile_name
        );
        return Ok(());
    };

    if is_profile_saved || yes_flag || currfiles_prohash.tracked_file_names.len() == 0 {
        return change_profile();
    };

    println!(
        "\ncurrent files ({}):",
        currfiles_prohash.tracked_file_names.len()
    );
    for filename in &currfiles_prohash.tracked_file_names {
        println!("  {}", filename);
    }

    let prompt = "The current files have not been saved or have been modified.\nThis action will delete them.\nAre you sure you want to proceed?";

    if utils::confirm(prompt) {
        return change_profile();
    } else {
        println!("\nProfile switch canceled.\n\n{}", profile_in_use_msg);
        return Ok(());
    }
}

pub fn discard_files(yes_flag: bool) -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_profile_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    if currfiles_prohash.tracked_file_names.len() == 0 {
        return Err(format!(
            "{}: Current files not found.\n\nNothing to discard.",
            APP_NAME
        ));
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
                eprintln!("{}: Error: Could not remove file: {}\n", APP_NAME, filename);
                return Err(err.to_string());
            }
        }

        println!("\nCurrent files discarded successfully!");
        return Ok(());
    };

    if is_profile_saved || yes_flag {
        return remove_current_files();
    }

    println!(
        "\ncurrent files ({}):",
        currfiles_prohash.tracked_file_names.len()
    );
    for filename in &currfiles_prohash.tracked_file_names {
        println!("  {}", filename);
    }

    let prompt = "The current files have not been saved or have been modified.\nThis action will delete them.\nAre you sure you want to proceed?";

    if utils::confirm(prompt) {
        return remove_current_files();
    } else {
        println!("\nNo files were discarded.");
        return Ok(());
    }
}

pub fn list() -> Result<(), String> {
    let app_paths = utils::get_app_paths();
    let gitconfig_data = git::get_gitconfig_data(&app_paths.gitconfig_file_path);

    let profile_dirs: Vec<String> =
        utils::get_profile_dirs(&app_paths.data_dir_path).unwrap_or_else(|_| vec![]);

    let currfiles_prohash = utils::get_profile_hash(&app_paths, gitconfig_data.file_exists, None)?;

    let mut current_profile_names: Vec<String> = vec![];

    println!("\n[saved profiles: {}]", profile_dirs.len());

    for profile_directory in profile_dirs {
        let mut prefix: &str = " ";
        let profile_prohash = utils::get_profile_hash(
            &app_paths,
            gitconfig_data.file_exists,
            Some(&profile_directory),
        )?;

        if currfiles_prohash.hash == profile_prohash.hash {
            prefix = "*";
            current_profile_names.push(profile_directory.clone());
        }

        println!("{} {}", prefix, profile_directory);
    }
    println!("");

    if currfiles_prohash.tracked_file_names.len() == 0 {
        println!("--- No profile in use ---");
        println!("Current files (.gitconfig and/or SSH keys) not found.");
    } else if current_profile_names.is_empty() {
        println!("--- No profile in use ---");
        println!("Current files have not been saved, or have been modified.");
    } else {
        println!("(current: {})", current_profile_names.join(" - "));
    }

    if gitconfig_data.file_exists {
        println!("  gitconfig name:  {:?}", gitconfig_data.name);
        println!("  gitconfig email: {:?}", gitconfig_data.email);
    }

    println!(
        "  current files ({}):",
        currfiles_prohash.tracked_file_names.len()
    );
    for filename in currfiles_prohash.tracked_file_names {
        println!("    {}", filename);
    }

    Ok(())
}

pub fn version() {
    println!("{}", VERSION);
}

pub fn help() {
    const HELP_TEXT: &str = r#"
xks - Git profile switcher with SSH key management

current_files: The default configuration files
stored in their default locations:
    ~/.gitconfig
    ~/.ssh/config
    ~/.ssh/id_ed25519  (also id_ed25519.pub)
    ~/.ssh/id_ecdsa    (also id_ecdsa.pub)
    ~/.ssh/id_rsa      (also id_rsa.pub)
    ~/.ssh/id_dsa      (also id_dsa.pub)

xks only manages these files.
    Other SSH keys or Git configuration files are ignored.

Usage:
    xks <command> [options]

Commands:
    save <profile>     Save current_files as a profile
    use <profile>      Apply a saved profile
    remove <profile>   Delete a saved profile
    discard            Delete current_files
    version            Show version number
    help               Show this help message

Options:
    -y                 Skip confirmation prompts

Examples:
    xks                # List saved profiles and current_files state
    xks save work      # Save current_files as 'work' profile
    xks use personal   # Switch to 'personal' profile
    xks use -          # Switch back to the previous profile
    xks remove alex    # Delete 'alex' profile
    xks discard        # Delete current_files

All data is stored in ~/.xks, including saved profiles.

For more details: https://xks.rwx222.com
"#;
    println!("{}", HELP_TEXT);
}
