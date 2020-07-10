#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate lazy_static;

mod backlight_controller;
mod command;

use self::command::Command;

fn main() {
    let cmd = Command::new();
    cmd.run();
}
