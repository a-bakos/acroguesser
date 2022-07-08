use crate::points::Points;
use crate::traits;
use crate::traits::Log;
use crate::Player;

#[derive(Debug)]
pub struct Gameplay {
    pub points: u8,
    pub history: Vec<String>,
    pub tries: u8,
    pub player: Player,
}

// Implement default Log trait for Game
impl traits::Log for Gameplay {}

impl Gameplay {
    pub fn new(player: Player) -> Self {
        let game = Self {
            history: vec![],
            points: 0,
            tries: 0,
            player,
        };
        game.status("New game started");

        game
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
        self.status("Tries ++");
        self.tries += 1;
    }

    pub fn add_to_guess_history(&mut self, acronym: String) {
        let status_msg = format!("Added to guess history: {}", &acronym);
        self.status(status_msg.as_str());
        self.history.push(acronym.to_lowercase());
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use crate::player::Player;

    use super::Gameplay;

    #[test]
    fn test_add_points() {
        let player: Player = Player::new(String::from("Frank"));
        let mut game: Gameplay = Gameplay::new(player);
        game.add_points(1);
        assert_eq!(5, game.points);
    }
}
