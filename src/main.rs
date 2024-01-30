mod backlight_controller;
mod command;

use self::command::Command;

fn main() {
    let cmd = Command::new();
    cmd.run();
}
