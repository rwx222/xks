use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct GitConfigData {
    pub name: String,
    pub email: String,
}

pub fn get_gitconfig_data(path: &PathBuf) -> GitConfigData {
    let mut name = String::new();
    let mut email = String::new();

    if let Ok(content) = fs::read_to_string(&path) {
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

    GitConfigData { name, email }
}
