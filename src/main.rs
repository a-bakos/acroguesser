mod consts;
mod gameplay;
mod gui;
mod journal;
mod points;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::points::Points;

fn main() {
    let player_name: String = String::from("abakos");
    let mut game = Gameplay::new(player_name);
    game.add_points(Points::Max);
    GUI::render(GUI::Start(&game.player_name));

    println!("{:?}", game);
}
