use crate::consts;
use crate::Journal;

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
