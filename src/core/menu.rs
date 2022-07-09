use crate::core::gui::GUI;
use crate::core::{consts, misc, traits};

pub enum Menu {
    Main,
    Start,
    Exit,
    Help,
}

impl traits::Log for Menu {}

impl Menu {
    pub fn menu_router() -> Menu {
        let user_command: String = misc::get_user_input();
        match user_command.as_str() {
            consts::CMD_QUIT_E | consts::CMD_QUIT_EXIT => {
                if Menu::confirm_exit() {
                    Menu::Exit
                } else {
                    Menu::Main
                }
            }
            consts::CMD_USER_H | consts::CMD_USER_HELP => Menu::Help,
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
        let user_command: String = misc::get_user_input();
        if user_command == consts::CMD_CONFIRM_Y || user_command == consts::CMD_CONFIRM_YES {
            GUI::render(GUI::Exiting);
            true
        } else {
            false
        }
    }
}
