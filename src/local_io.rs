use std::io;
use crate::{gui::GUI, consts};

pub fn get_player_name() -> String {
    let mut player_name = String::new();
    GUI::render(GUI::WaitingPlayerName);
    io::stdin()
        .read_line(&mut player_name)
        .expect(consts::ERROR_READING_PLAYER_NAME);
    player_name
}