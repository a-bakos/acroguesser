use crate::process_player_name;

pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new(player_name: String) -> Self {
        let player_name = process_player_name(player_name);

        Self { name: player_name }
    }
}
