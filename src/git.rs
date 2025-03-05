use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct GitConfigData {
    pub name: String,
    pub email: String,
    pub file_exists: bool,
}

pub fn get_gitconfig_data<T: AsRef<Path>>(path: T) -> GitConfigData {
    let mut name = String::new();
    let mut email = String::new();
    let mut file_exists: bool = false;

    if let Ok(content) = fs::read_to_string(&path) {
        file_exists = true;

        let mut in_user_section = false;

        for line in content.lines() {
            let trimmed = line.trim();

            if trimmed == "[user]" {
                in_user_section = true;
            } else if trimmed.starts_with('[') {
                in_user_section = false;
            }

            if in_user_section {
                if trimmed.starts_with("name =") {
                    name = trimmed.split('=').nth(1).unwrap().trim().to_string();
                } else if trimmed.starts_with("email =") {
                    email = trimmed.split('=').nth(1).unwrap().trim().to_string();
                }
            }
        }
    }

    GitConfigData {
        name,
        email,
        file_exists,
    }
}
