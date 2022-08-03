mod core;

use crate::core::gameplay::Gameplay;
use crate::core::gui::GUI;
use crate::core::menu::Menu;

fn main() {
    GUI::main_menu();

    'menu: loop {
        match Menu::menu_router() {
            Menu::Main => {
                GUI::main_menu();
                continue;
            }
            Menu::Start => Gameplay::new_game_init(),
            Menu::Help => {
                Gameplay::help();
                // continue;
            }
            Menu::Exit => {
                break 'menu;
            }
        };
    }
}
