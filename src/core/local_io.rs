use crate::core::{consts, gui::GUI};
use std::io;

pub fn get_player_name() -> String {
    GUI::render(GUI::WaitingPlayerName);
    let player_name: String = get_stdin().trim().to_string();
    player_name
}

pub fn get_user_input() -> String {
    let user_input: String = get_stdin().trim().to_lowercase().to_string();
    user_input
}

fn get_stdin() -> String {
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect(consts::ERROR_READING_INPUT);
    user_input
}
