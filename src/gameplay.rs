pub struct Gameplay {
    player_name: String,
    history: Vec<usize>,
    points: u8,
    tries: u8,
}

pub enum Points {
    Max = 10_u8,
    Med = 5_u8,
    Min = 1_u8,
}

// add_points
// store_in_history
// increase_tries
// store_player_name
