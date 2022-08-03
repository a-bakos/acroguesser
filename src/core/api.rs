use crate::core::consts;
use crate::core::journal;
use crate::core::journal::Journal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    acronym: String,
    title: TitleStruct,
}

#[derive(Debug, Deserialize, Serialize)]
struct TitleStruct {
    data: String,
}

pub fn req() -> Result<(), Box<dyn std::error::Error>> {
    let response: Vec<Test> = reqwest::blocking::get(consts::JOURNALS_API_ENDPOINT)?.json()?;
    println!("{:#?}", response);
    Ok(())
}
