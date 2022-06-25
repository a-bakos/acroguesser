use crate::consts;
use crate::gameplay::Gameplay;
use crate::misc;
use crate::Journal;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Journals {
    pub all: Vec<Journal>,
    pub used: Vec<Journal>,
    invalid: Vec<Journal>,
}

impl Journals {
    pub fn new() -> Self {
        Self {
            all: vec![
                Journal {
                    title: String::from("The title 1"),
                    acronym: "ABCD".to_lowercase(),
                },
                Journal {
                    title: String::from("The title 2"),
                    acronym: "EFGH".to_lowercase(),
                },
            ],
            used: vec![],
            invalid: vec![],
        }
    }

    //pub fn get_journal_list() {}
    //pub fn store_journal_list(&mut self) {}

    // move journal to used so it's not selected again to guess
    pub fn drop_journal(&mut self, journal: Journal) {}

    pub fn get_random_journal(&self, game: &Gameplay) -> &Journal {
        let selection: &Journal;
        let len = self.all.len();
        let mut rng = thread_rng();
        let index: u32 = rng.gen_range(0..len as u32);

        // machine chosen history

        selection = &self.all[index as usize];

        // check if journal is in history, get another one if so
        if selection.is_journal_in_history(&game.history) {
            println!("TODO select another one");
            // move journal out of list
        }

        if !misc::check_journal(selection) {
            println!("TODO select another one!");
            // move journal to invalid items
        }

        selection
    }
}
