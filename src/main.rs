#[macro_use]
extern crate async_trait;

#[macro_use]
extern crate failure;

#[macro_use]
extern crate lazy_static;

mod backlight_controller;
mod cmd;

use structopt::StructOpt;

fn main() { cmd::Command::from_args().run(); }
