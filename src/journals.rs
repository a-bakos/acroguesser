use crate::gameplay::Gameplay;
use crate::misc;
use crate::traits;
use crate::traits::Log;
use crate::Journal;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Journals {
    pub all: Vec<Journal>,
    pub used: Vec<Journal>,
    invalid: Vec<Journal>,
}

impl traits::Log for Journals {}

impl Journals {
    pub fn new() -> Self {
        Self {
            all: misc::populate_journals_list(),
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
        let index: u32 = thread_rng().gen_range(0..len as u32);

        // machine chosen history

        selection = &self.all[index as usize];

        // check if journal is in history, get another one if so
        if selection.is_journal_in_history(&game.history) {
            // move journal out of list
            // then select another one
            println!("TODO select another one");
        }

        if !misc::check_journal(selection) {
            // move journal to invalid items
            // then select another one
            println!("TODO select another one!");
        }

        self.status("Selection: {selection}");
        selection
    }
}
