use crate::gameplay::Gameplay;

mod gameplay;
mod journal;

fn main() {
    println!("Hello, world!");

    let game = Gameplay::new(String::from("abakos"));

    println!("{:?}", game);
}
