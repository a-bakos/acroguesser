pub enum GUI<'a> {
    Start(&'a str),
    End,
    WaitingPlayerName,
}

impl GUI<'_> {
    pub fn render(item: GUI) {
        match item {
            GUI::Start(player_name) => println!("Hello, {}!\n", player_name),
            GUI::End => println!("Bye!\n"),
            GUI::WaitingPlayerName => println!("Enter a player name:\n"),
        }
    }
}
