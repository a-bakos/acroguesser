#[derive(Debug)]
pub struct Gameplay {
    player_name: String,
    history: Vec<usize>,
    points: u8,
    tries: u8,
}

#[derive(Debug)]
pub enum Points {
    Max,
    Med,
    Min,
}

impl Points {
    pub fn get_the_points_value(points: Points) -> u8 {
        match points {
            Points::Max => 10,
            Points::Med => 5,
            Points::Min => 1,
        }
    }
}

// add_points
// store_in_history
// increase_tries
// store_player_name

impl Gameplay {
    pub fn new(player_name: String) -> Self {
        Self {
            player_name,
            history: vec![],
            points: 0,
            tries: 0,
        }
    }

    pub fn add_points(&mut self, points: Points) {
        self.points += Points::get_the_points_value(points);
    }
}
