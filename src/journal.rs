// use crate::consts;

#[derive(Debug)]
pub struct Journal {
    pub title: String,
    acronym_letters: [String; 4],
    pub acronym: String,
}

impl Journal {
    pub fn new() -> Self {
        Self {
            title: String::from("Hello title"),
            acronym: String::from("ABCD"),
            acronym_letters: [
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ],
        }
    }

    pub fn get_random_journal() -> Self {
        // TODO add logic here later; for now just return
        // a new instance
        Self::new()
    }
}

// get_journal_by_acronym
// get_journal_by_title
// validate journal
// is_title_ok
// is_acronym_ok
// get_random_journal
// get_journal_list
// store_journal_list
// is_letter_in_acronym
