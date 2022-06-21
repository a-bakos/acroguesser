use crate::Journal;

pub struct Journals {
    pub all: Vec<Journal>,
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
        }
    }

    //pub fn get_journal_list() {}
    //pub fn store_journal_list() {}

    pub fn get_random_journal(&self) -> &Journal {
        let _len = self.all.len();
        // TODO add logic here later; for now just return
        // a new instance

        // is journal in history

        // machine chosen history

        // Process random selection
        // - make sure its acro is 4 char len
        // - must have a title
        // - shouldn't start wit number

        &self.all[0]
    }
}
