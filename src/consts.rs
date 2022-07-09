// Mins, Maxs, Defaults, and Lengths

pub const DEBUG_VERBOSE: bool = true;

pub const MAX_TRIES: u8 = 5;

pub const VALID_ACRONYM_LEN: usize = 4;
pub const MIN_TITLE_LEN: usize = 3;

pub const ACRONYM_INVALID_START: &str = "0";

pub const DEFAULT_PLAYER_NAME: &str = "Player";

// Error handling

pub const ERROR_READING_PLAYER_NAME: &str = "Failed to read player name!";
pub const ERROR_READING_USER_GUESS: &str = "Failed to read user's guess!";
pub const ERROR_READING_INPUT: &str = "Failed to read input.";

// Menu and commands

pub const CMD_CONFIRM_Y: &str = "y";
pub const CMD_CONFIRM_YES: &str = "yes";

pub const CMD_QUIT_E: &str = "/e";
pub const CMD_QUIT_EXIT: &str = "/exit";

pub const CMD_USER_H: &str = "/h";
pub const CMD_USER_HELP: &str = "/help";

pub const CMD_USER_NAME: &str = "/name";

// File and IO

pub const FILE_PLAYER_DATA: &str = "player_data.txt";
pub const FILE_STATUS_DUMP: &str = "status_log.txt";

// API

pub const JOURNALS_API_ENDPOINT: &str = "";
