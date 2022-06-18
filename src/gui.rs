pub enum GUI<'a> {
    Start(&'a str),
    End,
    WaitingPlayerName,
    JournalTitle(&'a str),
}

impl GUI<'_> {
    pub fn render(item: GUI) {
        match item {
            GUI::Start(player_name) => println!("Hello, {}!\n", player_name),
            GUI::End => println!("Bye!\n"),
            GUI::WaitingPlayerName => println!("Enter a player name:\n"),
            GUI::JournalTitle(title) => println!("Here's the title: {}!\n", title),
        }
    }
}
