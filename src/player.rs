use crate::misc::process_player_name;
use crate::traits::{self, Log};

#[derive(Debug)]
pub struct Player {
    pub name: String,
}

impl traits::Log for Player {}

impl Player {
    pub fn new(player_name: String) -> Self {
        let player_name = process_player_name(player_name);

        let player = Self { name: player_name };

        let msg = format!("Player name: {}", &player.name);
        player.status(msg.as_str());

        player
    }
}
