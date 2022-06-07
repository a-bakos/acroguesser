use crate::gameplay::Gameplay;
use crate::gameplay::Points;

mod gameplay;
mod journal;

fn main() {
    println!("Hello, world!");

    let mut game = Gameplay::new(String::from("abakos"));
    game.add_points(Points::Max);

    println!("{:?}", game);
}
