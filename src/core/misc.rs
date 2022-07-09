use std::io;
use std::io::prelude::*;

use crate::core::consts;
use crate::core::journal::Journal;

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();
    // We want the cursor to stay at the end of the line,
    // so we print without a newline and flush manually.
    write!(stdout, "\nPress any key to continue...").unwrap();
    stdout.flush().unwrap();
    // Read a single byte and discard
    stdin.read(&mut [0u8]).unwrap();
}

pub fn process_player_name(player_name: String) -> String {
    let mut player_name = player_name.trim().to_string();
    if player_name.is_empty() {
        player_name = String::from(consts::DEFAULT_PLAYER_NAME);
    }
    player_name
}

pub fn increase_rounds_counter(rounds_counter: &mut u8) -> u8 {
    *rounds_counter + 1
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

    if acronym.is_empty() {
        return false;
    }

    // Acronym length check
    let acr_length: usize = acronym.chars().count();
    if acr_length != consts::VALID_ACRONYM_LEN {
        return false;
    }

    // Acronym starts with check
    if acronym.starts_with(consts::ACRONYM_INVALID_START) {
        return false;
    }

    true
}

pub fn populate_journals_list() -> Vec<Journal> {
    let container = vec![
        Journal {
            title: String::from("The title 1"),
            acronym: "ABCD".to_lowercase(),
        },
        Journal {
            title: String::from("The title 2"),
            acronym: "EFGH".to_lowercase(),
        },
    ];
    container
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

pub fn get_user_input() -> String {
    let mut user_command: String = String::new();
    io::stdin()
        .read_line(&mut user_command)
        .expect(consts::ERROR_READING_INPUT);
    let user_command: String = user_command.trim().to_lowercase().to_string();

    user_command
}

// Unit tests

#[cfg(test)]
mod tests {
    use crate::core::journal::Journal;

    use super::{
        check_journal_acronym, check_journal_title, increase_rounds_counter, process_player_name,
    };

    #[test]
    fn test_check_journal_title() {
        let journal: &Journal = &Journal {
            title: String::from("A title"),
            acronym: String::from("ABCD"),
        };
        assert!(check_journal_title(journal));
    }

    #[test]
    fn test_check_journal_acronym() {
        let journal: &Journal = &Journal {
            title: String::from("A title"),
            acronym: String::from("ABCD"),
        };
        assert!(check_journal_acronym(journal));
    }

    #[test]
    fn test_process_player_name() {
        let player_name: String = String::from("");
        assert_ne!(
            String::from(""),
            process_player_name(player_name),
            "Player name should have non-empty default value!"
        );
    }

    #[test]
    fn test_increase_rounds_counter() {
        let mut rounds_counter: u8 = 0;
        let expected: u8 = 1;
        assert_eq!(expected, increase_rounds_counter(&mut rounds_counter));
    }
}
