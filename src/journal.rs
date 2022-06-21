// use crate::consts;

#[derive(Debug)]
pub struct Journal {
    pub title: String,
    pub acronym: String,
}

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
        self.acronym.to_lowercase() == user_guess.to_lowercase()
    }

    pub fn is_journal_in_history(&self, game_history: &[String]) -> bool {
        game_history.contains(&self.acronym)
    }
}
