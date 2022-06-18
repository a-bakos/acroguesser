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

    loop {
        let journal = Journal::get_random_journal();
    }

    game.add_points(Points::Max);
    GUI::render(GUI::Start(&game.player_name));

    println!("{:?}", game);
}
