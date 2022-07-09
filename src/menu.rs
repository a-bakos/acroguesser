use crate::GUI;
use crate::{consts, traits};
use std::io;

pub enum Menu {}

impl traits::Log for Menu {}

impl Menu {
    pub fn show_main_menu() {
        println!("ACROGUESSER!\n");
        println!("\t(N)ew game\n");
        // println!("");
        println!("\t(E)xit\n");
    }

    pub fn exit_listener(user_command: &String) -> bool {
        if user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT {
            Menu::confirm_exit()
        } else {
            false
        }
    }

    pub fn confirm_exit() -> bool {
        GUI::render(GUI::Confirm);
        let mut user_command: String = String::new();
        io::stdin()
            .read_line(&mut user_command)
            .expect(consts::ERROR_READING_INPUT);
        let user_command: String = user_command.trim().to_lowercase().to_string();

        if user_command == consts::CMD_CONFIRM_Y || user_command == consts::CMD_CONFIRM_YES {
            GUI::render(GUI::Exiting);
            true
        } else {
            false
        }
    }
}
