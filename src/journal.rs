// use crate::consts;

#[derive(Debug)]
pub struct Journal {
    pub title: String,
    //acronym_letters: [String; 4],
    pub acronym: String,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            acronym: String::new(),
            // acronym_letters: [
            //     String::from("A"),
            //     String::from("B"),
            //     String::from("C"),
            //     String::from("D"),
            // ],
        }
    }

    pub fn is_letter_in_acronym(&self, letter: &str) -> bool {
        self.acronym.contains(letter)
    }

    pub fn is_matching_guess(&self, user_guess: &str) -> bool {
        self.acronym.to_lowercase() == user_guess.to_lowercase()
    }
}
