use crate::gameplay::Gameplay;
use crate::journals::Journals;
use crate::player::Player;
use crate::{consts, traits};
use crate::{local_io, GUI};
use std::io;

pub enum Menu {
    Main,
    Start,
    Exit,
}

// pub struct Menu;

impl traits::Log for Menu {}

impl Menu {
    pub fn main_menu() {
        GUI::render(GUI::MainMenu);
    }

    pub fn menu_router() -> Menu {
        let user_command: String = get_user_input();
        match user_command.as_str() {
            consts::CMD_QUIT_E | consts::CMD_QUIT_EXIT => {
                if Menu::confirm_exit() {
                    Menu::Exit
                } else {
                    Menu::Main
                }
            }
            consts::CMD_GAME_N | consts::CMD_GAME_NEW => Menu::Start,
            _ => Menu::Main,
        }
    }

    pub fn exit_listener(user_command: &String) -> bool {
        if user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT {
            Menu::confirm_exit()
        } else {
            false
        }
    }

    fn confirm_exit() -> bool {
        GUI::render(GUI::Confirm);
        let user_command: String = get_user_input();
        if user_command == consts::CMD_CONFIRM_Y || user_command == consts::CMD_CONFIRM_YES {
            GUI::render(GUI::Exiting);
            true
        } else {
            false
        }
    }
}

fn get_user_input() -> String {
    let mut user_command: String = String::new();
    io::stdin()
        .read_line(&mut user_command)
        .expect(consts::ERROR_READING_INPUT);
    let user_command: String = user_command.trim().to_lowercase().to_string();

    user_command
}

fn new_game_init() {
    // player setup
    let player_name: String = local_io::get_player_name();
    let player: Player = Player::new(player_name);

    // game loop setup
    let mut game = Gameplay::new(player);
    let mut journals = Journals::new(); // init journals list
    GUI::render(GUI::Start(&game.player.name));
    //file::write_player_data(&game);
}
