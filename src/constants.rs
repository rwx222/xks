pub const VERSION: &str = "1.0.1";

pub const APP_NAME: &str = "xks";
pub const DATA_DIR_NAME: &str = ".xks";
pub const CONFIG_DIR_NAME: &str = ".config";

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

pub const PREVIOUS_PROFILE_FILE_NAME: &str = "previous_profile";

pub const YES_FLAG: &str = "-y";

pub const TOGGLE_PREV: &str = "-";

pub const HELP_LINE: &str = "See:\n    xks help";

pub const PROFILE_NAME_MAX_LENGTH: usize = 50;

pub const REMOVING_DIR_ERR: &str =
    "Error: Could not remove directory. This may be due to insufficient permissions.";
pub const READING_DIR_ERR: &str =
    "Error: Could not read directory. This may be due to insufficient permissions.";
pub const READING_HASH_FILES_ERR: &str = "Error: Could not get files hash.";
