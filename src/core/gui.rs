use colorful::{Color, Colorful};

pub enum GUI<'a> {
    Start(&'a str),
    End,
    WaitingPlayerName,
    WaitingGuess,
    JournalTitle(&'a str),
    Win,
    TryAgain,
    EmptyInput,
    MaxTriesReached,
    YourGuess(&'a str),
    Confirm,
    Exiting,
    MainMenu,
    HelpMenu,
}

impl GUI<'_> {
    pub fn render(item: GUI) {
        match item {
            GUI::Start(player_name) => println!("Hello, {}!\n", player_name),
            GUI::End => println!("Game over!\n"),
            GUI::WaitingPlayerName => println!("Enter a player name:\n"),
            GUI::WaitingGuess => println!("Your guess: "),
            GUI::JournalTitle(title) => {
                println!("Here's the title: {}!\n", title.bg_color(Color::Yellow))
            }
            GUI::Win => println!("WINNER! You guessed it correctly!"),
            GUI::TryAgain => println!("Try again!"),
            GUI::MaxTriesReached => {
                println!("You've reached the maximum number of tries and lost this round. Here's another one to guess.")
            }
            GUI::EmptyInput => println!("Empty input. Type in your guess."),
            GUI::YourGuess(guess) => println!("Your guess was: {}", guess),
            GUI::Confirm => println!("-Y or -YES to confirm: "),
            GUI::Exiting => println!("Exiting..."),
            GUI::MainMenu => {
                println!("ACROGUESSER!\n");
                println!("\t-N: New game\n");
                println!("\t-H: Instructions\n");
                println!("\t-E: Exit\n");

                // waiting for user choice
                println!(">> ");
            }
            GUI::HelpMenu => {
                println!("HOW TO PLAY?\n");
                println!("instructions here...");
            }
        }
    }

    pub fn main_menu() {
        GUI::render(GUI::MainMenu);
    }

    pub fn help_menu() {
        GUI::render(GUI::HelpMenu);
    }
}
