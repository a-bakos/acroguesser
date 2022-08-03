use crate::core::consts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RawJournal {
    pub acronym: String,
    pub title: RawJournalTitle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawJournalTitle {
    pub data: String,
}

pub fn get_journal_list() -> Vec<RawJournal> {
    let temps: Vec<RawJournal> = req().unwrap();
    temps
}

fn req() -> Result<Vec<RawJournal>, Box<dyn std::error::Error>> {
    let response: Vec<RawJournal> =
        reqwest::blocking::get(consts::JOURNALS_API_ENDPOINT)?.json()?;
    Ok(response)
}
