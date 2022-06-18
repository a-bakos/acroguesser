mod consts;
mod gameplay;
mod gui;
mod journal;
mod points;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::journal::Journal;
use crate::points::Points;

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
        game.add_points(Points::Max);
        println!("{:?}", game);

        break;
    }

    GUI::render(GUI::End);
}
