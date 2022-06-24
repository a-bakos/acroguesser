use crate::consts;
use crate::GUI;
use std::io;

pub fn get_player_name() -> String {
    let mut player_name = String::new();
    GUI::render(GUI::WaitingPlayerName);
    io::stdin()
        .read_line(&mut player_name)
        .expect(consts::ERROR_READING_PLAYER_NAME);
    player_name
}

pub fn process_player_name(player_name: String) -> String {
    let mut player_name = player_name.trim().to_string();
    if player_name.is_empty() {
        player_name = String::from(consts::DEFAULT_PLAYER_NAME);
    }
    player_name
}

// WIP
fn exit_listener(user_command: String) -> bool {
    user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT
}
