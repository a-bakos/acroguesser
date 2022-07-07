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

#[cfg(test)]
mod tests {
    use super::Journal;

    #[test]
    fn test_is_letter_in_acronym() {
        let journal: Journal = Journal {
            title: String::from("Hello title"),
            acronym: String::from("ABCD"),
        };
        assert!(journal.is_letter_in_acronym("A"));
    }

    #[test]
    fn test_is_matching_guess() {
        let journal: Journal = Journal {
            title: String::from("Hello title"),
            acronym: String::from("ABCD"),
        };
        let user_guess: &str = "ABCD";
        assert!(journal.is_matching_guess(user_guess));
    }

    #[test]
    fn test_is_journal_in_history() {
        let journal: Journal = Journal {
            title: String::from("Hello title"),
            acronym: String::from("ABCD"),
        };
        let game_history: &[String] = &[String::from("ABCD")];
        assert!(journal.is_journal_in_history(game_history));
    }
}
