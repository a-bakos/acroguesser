use std::io;

use crate::core::gui::GUI;
use crate::core::journal::Journal;
use crate::core::journals::Journals;
use crate::core::menu::Menu;
use crate::core::player::Player;
use crate::core::points::Points;
use crate::core::traits::Log;
use crate::core::{consts, local_io, misc, traits};

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

    pub fn new_game_init() {
        misc::clear_terminal();
        // player setup
        let player_name: String = local_io::get_player_name();
        let player: Player = Player::new(player_name);

        // game loop setup
        let mut game = Gameplay::new(player);
        let mut journals = Journals::new(); // init journals list
        GUI::render(GUI::Start(&game.player.name));
        //file::write_player_data(&game);

        // gameloop start
        // outer loop for main rounds
        'mainGameloop: loop {
            let mut rounds_counter: u8 = 0;
            // get a journal to guess
            let journal: Journal = Journals::get_random_journal(&mut journals, &game);

            'guessRound: loop {
                GUI::render(GUI::JournalTitle(&journal.title));

                if rounds_counter == consts::MAX_TRIES {
                    GUI::render(GUI::MaxTriesReached);
                    break 'guessRound;
                }

                // deal with guess input
                let mut user_guess: String = String::new();
                GUI::render(GUI::WaitingGuess);
                io::stdin()
                    .read_line(&mut user_guess)
                    .expect(consts::ERROR_READING_USER_GUESS);
                let user_guess: String = user_guess.trim().to_string();

                if user_guess.is_empty() {
                    GUI::render(GUI::EmptyInput);
                } else {
                    if Menu::exit_listener(&user_guess) {
                        break 'guessRound;
                    }

                    GUI::render(GUI::YourGuess(&user_guess));

                    if journal.is_matching_guess(&user_guess) {
                        game.add_points(rounds_counter);
                        GUI::render(GUI::Win);
                        journals.drop_journal(journal);
                        break 'guessRound;
                    } else {
                        GUI::render(GUI::TryAgain);
                    }

                    game.increase_tries();
                    game.add_to_guess_history(user_guess);
                    rounds_counter = misc::increase_rounds_counter(&mut rounds_counter);
                }
            }

            println!("{:?}", game);

            // temp break
            break 'mainGameloop;
        }
        GUI::render(GUI::End);

        misc::clear_terminal();
        GUI::main_menu();
    }

    pub fn help() {
        misc::clear_terminal();
        GUI::help_menu();
        misc::pause();
    }
}

// Unit tests

#[cfg(test)]
mod tests {
    use crate::core::player::Player;

    use super::Gameplay;

    #[test]
    fn test_add_points() {
        let player: Player = Player::new(String::from("Frank"));
        let mut game: Gameplay = Gameplay::new(player);
        game.add_points(1);
        assert_eq!(5, game.points);
    }
}
