mod consts;
mod gameplay;
mod gui;
mod journal;
mod journals;
mod points;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::journal::Journal;
use crate::journals::Journals;

use std::io;

fn main() {
    // player setup
    let mut player_name: String = String::new();
    GUI::render(GUI::WaitingPlayerName);
    io::stdin()
        .read_line(&mut player_name)
        .expect(consts::ERROR_READING_PLAYER_NAME);

    player_name = player_name.trim().to_string();
    if player_name.is_empty() {
        player_name = String::from(consts::DEFAULT_PLAYER_NAME);
    }

    // game loop setup
    let mut game = Gameplay::new(player_name);
    println!("{:?}", game);
    let journals = Journals::new();
    GUI::render(GUI::Start(&game.player_name));

    // gameloop start
    // outer loop for main rounds
    loop {
        let mut rounds_counter: u8 = 0;

        // get a journal to guess
        // journal setup
        let mut journal: &Journal = Journals::get_random_journal(&journals);
        // check if journal is in history, get another one if so
        if journal.is_journal_in_history(&game.history) {
            journal = Journals::get_random_journal(&journals);
        }

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
                    break;
                } else {
                    GUI::render(GUI::TryAgain);
                }

                game.increase_tries();
                game.store_in_guess_history(user_guess);
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

fn exit_listener(user_command: String) -> bool {
    user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT
}
