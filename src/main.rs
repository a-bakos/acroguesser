mod consts;
mod file;
mod gameplay;
mod gui;
mod journal;
mod journals;
mod misc;
mod player;
mod points;
mod local_io;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::journal::Journal;
use crate::journals::Journals;
use crate::player::Player;

use std::io;

fn main() {
    // player setup
    let player_name: String = local_io::get_player_name();
    let player: Player = Player::new(player_name);

    // game loop setup
    let mut game = Gameplay::new(player);
    let journals = Journals::new(); // init journals list
    GUI::render(GUI::Start(&game.player.name));

    file::write_player_data(&game);

    // gameloop start
    // outer loop for main rounds
    loop {
        let mut rounds_counter: u8 = 0;

        // get a journal to guess
        let journal: &Journal = Journals::get_random_journal(&journals, &game);

        loop {
            GUI::render(GUI::JournalTitle(&journal.title));

            if rounds_counter == consts::MAX_TRIES {
                GUI::render(GUI::MaxTriesReached);
                break;
            }

            // deal with guess input
            let mut user_guess: String = String::new();
            io::stdin()
                .read_line(&mut user_guess)
                .expect(consts::ERROR_READING_USER_GUESS);
            let user_guess: String = user_guess.trim().to_string();

            if !user_guess.is_empty() {
                GUI::render(GUI::YourGuess(&user_guess));

                if journal.is_matching_guess(&user_guess) {
                    game.add_points(rounds_counter);
                    GUI::render(GUI::Win);
                    // todo move journal to used journals
                    break;
                } else {
                    GUI::render(GUI::TryAgain);
                }

                game.increase_tries();
                game.add_to_guess_history(user_guess);
                rounds_counter += 1;
            } else {
                GUI::render(GUI::EmptyInput);
            }
        }

        println!("{:?}", game);

        // temp break
        break;
    }

    GUI::render(GUI::End);
}
