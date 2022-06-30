use crate::points::Points;
use crate::Player;

pub trait Log {
    fn print_info(&self) {
        println!("Hello from trait");
    }
}

#[derive(Debug)]
pub struct Gameplay {
    pub points: u8,
    pub history: Vec<String>,
    pub tries: u8,
    pub player: Player,
}

// Implement default Log trait for Game
impl Log for Gameplay {
    fn print_info(&self) {
        println!("Points: {}", self.points);        
    }
}

impl Gameplay {
    pub fn new(player: Player) -> Self {
        Self {
            history: vec![],
            points: 0,
            tries: 0,
            player,
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

    pub fn add_to_guess_history(&mut self, acronym: String) {
        self.history.push(acronym.to_lowercase());
    }

}
