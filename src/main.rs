mod backlight_controller;
mod command;
mod shadow {
    #![allow(clippy::needless_raw_string_hashes)]
    use shadow_rs::shadow;
    shadow!(build);

    pub use self::build::*;
}

use self::command::Cli;

fn main() {
    let exit_code = Cli::default().run();
    std::process::exit(exit_code);
}
