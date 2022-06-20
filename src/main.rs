mod consts;
mod gameplay;
mod gui;
mod journal;
mod points;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::journal::Journal;

use std::io;

fn main() {
    let mut player_name: String = String::new();
    GUI::render(GUI::WaitingPlayerName);
    io::stdin()
        .read_line(&mut player_name)
        .expect("Failed to read player name!");

    let player_name: String = player_name.trim().to_string();

    let mut game = Gameplay::new(player_name);
    GUI::render(GUI::Start(&game.player_name));
    // game initialised
    println!("{:?}", game);

    loop {
        // get a journal to guess
        let journal = Journal::get_random_journal();
        GUI::render(GUI::JournalTitle(&journal.title));

        let mut rounds_counter: u8 = 0;
        loop {
            if rounds_counter == consts::MAX_TRIES {
                GUI::render(GUI::MaxTriesReached);
                break;
            }

            let mut user_guess: String = String::new();
            io::stdin()
                .read_line(&mut user_guess)
                .expect("Failed to read user's guess!");

            let user_guess: String = user_guess.trim().to_string();
            if user_guess.len() >= 1 {
                println!("Your guess: {}", user_guess);

                if journal.is_matching_guess(&user_guess) {
                    game.add_points(rounds_counter);
                    GUI::render(GUI::Win);
                    break;
                } else {
                    GUI::render(GUI::TryAgain);
                }

                game.increase_tries();
                game.store_in_history(user_guess);
                rounds_counter += 1;
            } else {
                GUI::render(GUI::EmptyInput);
            }
        }

        println!("{:?}", game);

        break;
    }

    GUI::render(GUI::End);
}
