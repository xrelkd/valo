use std::io::Write;

use clap::{CommandFactory, Parser, Subcommand};
use futures::FutureExt;
use tokio::signal::unix::{signal, SignalKind};

use crate::{
    backlight_controller::{Backlight, KeyboardBacklight, ScreenBacklight},
    shadow,
};

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

#[derive(Parser)]
#[command(
    name = "valo",
    author,
    version,
    long_version = shadow::CLAP_LONG_VERSION,
    about,
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    #[clap(about = "Print version information")]
    Version,

    #[clap(about = "Output shell completion code for the specified shell (bash, zsh, fish)")]
    Completions { shell: clap_complete::Shell },

    #[clap(name = "screen", about = "Change screen backlight level.")]
    ScreenBacklight {
        #[clap(subcommand, name = "screen")]
        command: BacklightCommands,
    },

    #[clap(name = "keyboard", about = "Change MacBooks' keyboard backlight level.")]
    KeyboardBacklight {
        #[clap(subcommand, name = "keyboard")]
        command: BacklightCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum BacklightCommands {
    #[clap(about = "Get current keyboard backlight brightness value")]
    Get,

    #[clap(about = "Get current keyboard backlight brightness percentage value")]
    GetPercentage,

    #[clap(about = "Set backlight brightness as value")]
    Set { value: u64 },

    #[clap(about = "Set backlight brightness as percentage value")]
    SetPercentage { percentage_value: u64 },

    #[clap(about = "Increase backlight brightness by percentage value")]
    Up {
        #[clap(default_value = "5")]
        percentage_value: u64,
    },

    #[clap(about = "Decrease backlight brightness by percentage value")]
    Down {
        #[clap(default_value = "5")]
        percentage_value: u64,
    },

    #[clap(about = "Set backlight brightness as max")]
    Max,

    #[clap(about = "Turn off backlight")]
    Off,

    #[clap(about = "Perform breathing light mode")]
    BreathingLight {
        #[arg(long, help = "percentage per step")]
        step: u64,

        #[arg(long, help = "delay millisecond")]
        delay: u64,
    },
}

impl Default for Cli {
    fn default() -> Self { Self::parse() }
}

impl Cli {
    pub fn run(self) -> i32 {
        match self.commands {
            Commands::ScreenBacklight { command } => tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move { command.screen().await }),
            Commands::KeyboardBacklight { command } => tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async move { command.keyboard().await }),
            Commands::Completions { shell } => {
                let mut app = Self::command();
                let bin_name = app.get_name().to_string();
                clap_complete::generate(shell, &mut app, bin_name, &mut std::io::stdout());
                EXIT_SUCCESS
            }
            Commands::Version => {
                std::io::stdout()
                    .write_all(Self::command().render_long_version().as_bytes())
                    .expect("Failed to write to stdout");
                EXIT_SUCCESS
            }
        }
    }
}

impl BacklightCommands {
    pub async fn keyboard(self) -> i32 {
        match KeyboardBacklight::new().await {
            Ok(backlight) => self.run(backlight).await,
            Err(err) => {
                eprintln!("{err}");
                EXIT_FAILURE
            }
        }
    }

    pub async fn screen(self) -> i32 {
        match ScreenBacklight::new().await {
            Ok(backlight) => self.run(backlight).await,
            Err(err) => {
                eprintln!("{err}");
                EXIT_FAILURE
            }
        }
    }

    #[allow(clippy::future_not_send, clippy::never_loop)]
    async fn run<B>(self, mut backlight: B) -> i32
    where
        B: Backlight,
    {
        let current_brightness_value = match self {
            Self::Get => {
                let v = backlight.current_value();
                println!("{v}");
                Ok(v)
            }
            Self::GetPercentage => {
                let v = backlight.current_percentage();
                println!("{v}");
                Ok(v)
            }
            Self::Set { value } => backlight.set(value).await,
            Self::SetPercentage { percentage_value } => {
                backlight.set_percentage(percentage_value).await
            }
            Self::Up { percentage_value } => backlight.up(percentage_value).await,
            Self::Down { percentage_value } => backlight.down(percentage_value).await,
            Self::Off => backlight.off().await,
            Self::Max => backlight.max().await,
            Self::BreathingLight { step, delay } => {
                let mut term_signal = signal(SignalKind::terminate()).unwrap();
                let mut int_signal = signal(SignalKind::interrupt()).unwrap();

                let signal_receiver = async {
                    futures::select! {
                        _ = term_signal.recv().fuse() => {},
                        _ = int_signal.recv().fuse() => {},
                    }
                };

                let origin_value = backlight.current_value();

                loop {
                    futures::select! {
                        () = backlight.run_breathing(step, delay, 0, 100).fuse() => {
                            break;
                        },
                        () = signal_receiver.fuse() => {
                            break;
                        },
                    }
                }

                backlight.set(origin_value).await
            }
        };

        current_brightness_value.map(|_value| EXIT_SUCCESS).unwrap_or(EXIT_FAILURE)
    }
}
