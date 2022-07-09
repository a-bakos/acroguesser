use crate::GUI;
use crate::{consts, traits};
use std::io;

pub struct Menu;

impl traits::Log for Menu {}

impl Menu {
    pub fn main_menu() {
        // GUI
        println!("ACROGUESSER!\n");
        println!("\t\\N: New game\n");
        // println!("");
        println!("\t\\E: Exit\n");

        // waiting for user choice
        println!(">> ");
        let user_command: String = get_user_input();
        Menu::menu_router(user_command);
    }

    fn menu_router(user_command: String) {
        match user_command.as_str() {
            consts::CMD_QUIT_E | consts::CMD_QUIT_EXIT => {
                println!("FULL EXIT")
            }
            _ => {
                println!("something else")
            }
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
