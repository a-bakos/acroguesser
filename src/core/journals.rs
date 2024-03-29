use crate::core::gameplay::Gameplay;
use crate::core::journal::Journal;
use crate::core::misc;
use crate::core::traits;
use crate::core::traits::Log;
use rand::{thread_rng, Rng};
use urlencoding::decode;

use crate::core::api;

#[derive(Debug)]
pub struct Journals {
    pub all: Vec<Journal>,
    pub used: Vec<Journal>,
    //invalid: Vec<Journal>,
}

impl traits::Log for Journals {}

impl Journals {
    pub fn new() -> Self {
        Self {
            all: Journals::populate_journals_list(),
            used: vec![],
            //invalid: vec![],
        }
    }

    fn populate_journals_list() -> Vec<Journal> {
        let journals = api::get_journal_list();
        let mut container = vec![];

        for journal in journals.into_iter() {
            let decoded_title: String = decode(&journal.title.data).expect("UTF-8").into_owned();
            let new_journal = Journal::new(journal.acronym, decoded_title);
            container.push(new_journal);
        }

        container
    }

    // move journal to used so it's not selected again to guess
    pub fn drop_journal(&mut self, journal: Journal) {
        self.used.insert(0, journal);
    }

    // TODO return Option<Journal> instead!
    pub fn get_random_journal(&mut self, game: &Gameplay) -> Journal {
        // machine chosen item
        let len = self.all.len();
        let index: u32 = thread_rng().gen_range(0..len as u32);
        let selection: Journal = self.all.remove(index as usize);

        // check if journal is in history, get another one if so
        if selection.is_journal_in_history(&game.history) {
            // move journal out of list
            // then select another one
            println!("TODO select another one");
        }

        if !misc::check_journal(&selection) {
            // move journal to invalid items
            // then select another one
            println!("TODO select another one!");
        }

        let status_msg = format!("Selection: {}", &selection.acronym);
        self.status(status_msg.as_str());

        selection
    }
}
