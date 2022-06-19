use crate::points::Points;

#[derive(Debug)]
pub struct Gameplay {
    pub player_name: String,
    pub points: u8,
    pub history: Vec<String>,
    pub tries: u8,
}

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
        self.points += Points::add_points_value(points);
    }

    pub fn increase_tries(&mut self) {
        self.tries += 1;
    }

    pub fn store_in_history(&mut self, acronym: String) {
        self.history.push(acronym);
    }

    // store_player_name
}
