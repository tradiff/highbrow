use std::process::Command;

use crate::utils::show_error_dialog;

pub fn spawn_browser(cmd: &str, url: &str) {
    let mut parts = cmd.split_whitespace();

    if let Some(program) = parts.next() {
        let mut command = Command::new(program);
        command.args(parts).arg(url);

        if let Err(e) = command.spawn() {
            show_error_dialog(&format!("Failed to launch {}: {}", cmd, e));
        }
    }
}
