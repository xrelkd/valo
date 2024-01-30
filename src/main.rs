mod backlight_controller;
mod command;
mod error;
mod shadow {
    #![allow(clippy::needless_raw_string_hashes)]
    use shadow_rs::shadow;
    shadow!(build);

    pub use self::build::*;
}

use self::command::Cli;

fn main() {
    if let Err(err) = Cli::default().run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
