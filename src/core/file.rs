use crate::core::consts;
use crate::core::gameplay::Gameplay;

use std::fs;

pub fn write_player_data(game: &Gameplay) {
    let mut to_write = String::new();
    to_write.push_str("PLAYER DATA:\n");
    to_write.push_str(&game.player.name);
    to_write.push_str("\n");

    fs::write(consts::FILE_PLAYER_DATA, to_write).ok();
}

pub fn write_to_log(log_msg: &str) {
    fs::write(consts::FILE_STATUS_DUMP, String::from(log_msg)).ok();
}
