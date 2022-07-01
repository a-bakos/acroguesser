// use crate::consts;
use crate::traits::{self, Log};

#[derive(Debug)]
pub struct Journal {
    pub title: String,
    pub acronym: String,
}

impl traits::Log for Journal {}

impl Journal {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            acronym: String::new(),
        }
    }

    pub fn is_letter_in_acronym(&self, letter: &str) -> bool {
        self.acronym.contains(letter)
    }

    pub fn is_matching_guess(&self, user_guess: &str) -> bool {
        self.status("Matching guess");
        self.acronym.to_lowercase() == user_guess.to_lowercase()
    }

    pub fn is_journal_in_history(&self, game_history: &[String]) -> bool {
        self.status("In game history already");
        game_history.contains(&self.acronym)
    }
}
