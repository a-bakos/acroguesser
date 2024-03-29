use crate::core::consts;
use chrono::prelude::*;
use std::fs;
use std::io::Write;

pub trait Log {
    fn status(&self, msg: &str) {
        if consts::DEBUG_VERBOSE {
            let mut log_msg: String = String::new();
            log_msg.push_str(msg);

            let mut file = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(consts::FILE_STATUS_DUMP)
                .unwrap();

            let time: DateTime<Utc> = Utc::now();

            writeln!(file, "{} {}", time, log_msg).ok();
        }
    }
}
