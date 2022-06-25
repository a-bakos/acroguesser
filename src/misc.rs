use crate::consts;
use crate::Journal;
use crate::GUI;
use std::io;

pub fn get_player_name() -> String {
    let mut player_name = String::new();
    GUI::render(GUI::WaitingPlayerName);
    io::stdin()
        .read_line(&mut player_name)
        .expect(consts::ERROR_READING_PLAYER_NAME);
    player_name
}

pub fn process_player_name(player_name: String) -> String {
    let mut player_name = player_name.trim().to_string();
    if player_name.is_empty() {
        player_name = String::from(consts::DEFAULT_PLAYER_NAME);
    }
    player_name
}

// WIP
fn exit_listener(user_command: String) -> bool {
    user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT
}

pub fn check_journal(journal: &Journal) -> bool {
    if !check_journal_acronym(&journal) {
        return false;
    }

    if !check_journal_title(&journal) {
        return false;
    }

    true
}

/**
 * Journal acronym checker method
 */
fn check_journal_acronym(journal: &Journal) -> bool {
    let acronym: &str = journal.acronym.trim();

    // Acronym length check
    let acr_length: usize = acronym.chars().count();
    if acr_length != consts::VALID_ACRONYM_LEN {
        return false;
    }

    // Acronym starts with check
    if acronym.starts_with("0") {
        return false;
    }

    true
}

/**
 * Journal title checker method
 */
fn check_journal_title(journal: &Journal) -> bool {
    if journal.title.is_empty() || consts::MIN_TITLE_LEN >= journal.title.trim().chars().count() {
        return false;
    }
    true
}
