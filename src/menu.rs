use crate::consts;

// wip
fn exit_listener(user_command: String) -> bool {
    user_command == consts::CMD_QUIT_E || user_command == consts::CMD_QUIT_EXIT
}

