// use crate::consts;

pub struct Journal {
    title: String,
    acronym: [String; 4],
}

impl Journal {
    pub fn new() -> Self {
        Self {
            title: String::from("Hello title"),
            acronym: [
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
            ],
        }
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
