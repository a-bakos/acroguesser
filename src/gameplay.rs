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

    pub fn add_points(&mut self, rounds: u8) {
        let points: Points = self.calculate_points(rounds);
        self.points += Points::get_points_value(points);
    }

    fn calculate_points(&self, rounds: u8) -> Points {
        let points: Points = match rounds {
            1 => Points::Round1,
            2 => Points::Round2,
            3 => Points::Round3,
            4 => Points::Round4,
            5 => Points::Round5,
            _ => Points::NoPoint,
        };
        points
    }

    pub fn increase_tries(&mut self) {
        self.tries += 1;
    }

    pub fn store_in_history(&mut self, acronym: String) {
        self.history.push(acronym);
    }

    // store_player_name
}
