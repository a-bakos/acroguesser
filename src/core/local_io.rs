use crate::core::{consts, gui::GUI};
use std::io;

// TODO deal with custom error messages
pub fn get_player_name() -> String {
    GUI::render(GUI::WaitingPlayerName);
    get_stdin().trim().to_string()
}

// TODO return Options<String> ??
// TODO deal with custom error messages
fn get_stdin() -> String {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect(consts::ERROR_READING_INPUT);
    user_input
}

pub fn get_user_input() -> String {
    let user_input: String = get_stdin().trim().to_lowercase();
    user_input
}

// TODO deal with custom error messages
pub fn get_user_guess() -> String {
    GUI::render(GUI::WaitingGuess);
    get_stdin().trim().to_string()
    // consts::ERROR_READING_USER_GUESS
}
