use crate::consts;
use crate::Journal;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Journals {
    pub all: Vec<Journal>,
    pub used: Vec<Journal>,
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
                Journal::new(),
            ],
            used: vec![],
        }
    }

    //pub fn get_journal_list() {}
    //pub fn store_journal_list(&mut self) {}

    // move journal to used so it's not selected again to guess
    pub fn drop_journal(&mut self, journal: Journal) {}

    pub fn get_random_journal(&self) -> &Journal {
        let selection: &Journal;
        let len = self.all.len();
        let mut rng = thread_rng();
        let index: u32 = rng.gen_range(0..len as u32);

        // is journal in history

        // machine chosen history

        // Process random selection
        // - make sure its acro is 4 char len
        // - must have a title
        // - shouldn't start wit number

        selection = &self.all[index as usize];

        if selection.title.is_empty() || selection.acronym.is_empty() {
            println!("select another one");
        }

        selection
    }
}
