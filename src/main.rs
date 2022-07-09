mod consts;
mod file;
mod gameplay;
mod gui;
mod journal;
mod journals;
mod local_io;
mod menu;
mod misc;
mod player;
mod points;
mod traits;

use crate::gameplay::Gameplay;
use crate::gui::GUI;
use crate::journal::Journal;
use crate::menu::Menu;
use crate::player::Player;

fn main() {
    GUI::main_menu();

    'menu: loop {
        let menu: Menu = Menu::menu_router();
        match menu {
            Menu::Main => {
                GUI::main_menu();
                continue;
            }
            Menu::Start => Gameplay::new_game_init(),
            Menu::Exit => break 'menu,
        }
    }
}
