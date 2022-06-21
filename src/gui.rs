pub enum GUI<'a> {
    Start(&'a str),
    End,
    WaitingPlayerName,
    JournalTitle(&'a str),
    Win,
    TryAgain,
    EmptyInput,
    MaxTriesReached,
    YourGuess(&'a str),
}

impl GUI<'_> {
    pub fn render(item: GUI) {
        match item {
            GUI::Start(player_name) => println!("Hello, {}!\n", player_name),
            GUI::End => println!("Bye!\n"),
            GUI::WaitingPlayerName => println!("Enter a player name:\n"),
            GUI::JournalTitle(title) => println!("Here's the title: {}!\n", title),
            GUI::Win => println!("WINNER! You guessed it correctly!"),
            GUI::TryAgain => println!("Try again!"),
            GUI::MaxTriesReached => {
                println!("You've reached the maximum number of tries and lost this round. Here's another one to guess.")
            }
            GUI::EmptyInput => println!("Empty input. Type in your guess."),
            GUI::YourGuess(guess) => println!("Your guess was: {}", guess),
        }
    }
}
