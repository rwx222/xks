pub const VERSION: &str = "0.1.0";

pub const DATA_DIR_NAME: &str = ".xks";
pub const SSH_DIR: &str = ".ssh";

pub const GITCONFIG_FILE_NAME: &str = ".gitconfig";
pub const TRACKED_FILE_NAMES: [&str; 10] = [
    GITCONFIG_FILE_NAME,
    "config",
    "id_ed25519",
    "id_ed25519.pub",
    "id_ecdsa",
    "id_ecdsa.pub",
    "id_rsa",
    "id_rsa.pub",
    "id_dsa",
    "id_dsa.pub",
];

pub const YES_FLAG: &str = "-y";

pub const PROFILE_NAME_MAX_LENGTH: usize = 50;
pub const USAGE_LINE: &str = "Example: xks save alex_github";
pub const VALID_LINE: &str = "Example: xks save alex_2@wi-fi.org";
pub const HELP_LINE: &str = "Run:\n  xks help \nTo see usage instructions.";

pub const REMOVING_DIR_ERR: &str =
    "Error: Could not remove directory. This may be due to insufficient permissions.";
pub const READING_DIR_ERR: &str =
    "Error: Could not read directory. This may be due to insufficient permissions.";
pub const READING_HASH_FILES_ERR: &str = "Error: Could not read hash files.";
