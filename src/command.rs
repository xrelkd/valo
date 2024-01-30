use futures::FutureExt;
use structopt::{clap::Shell as ClapShell, StructOpt};
use tokio::signal::unix::{signal, SignalKind};

use crate::backlight_controller::{Backlight, KeyboardBacklight, ScreenBacklight};

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "screen", about = "Changes screen backlight level.")]
    ScreenBacklight {
        #[structopt(subcommand, name = "screen")]
        cmd: BacklightCommand,
    },

    #[structopt(name = "keyboard", about = "Changes MacBooks' keyboard backlight level.")]
    KeyboardBacklight {
        #[structopt(subcommand, name = "keyboard")]
        cmd: BacklightCommand,
    },

    #[structopt(name = "completions", about = "Generates tab-completion scripts for your shell")]
    Completions {
        #[structopt(name = "shell")]
        shell: ClapShell,
    },

    #[structopt(name = "version", about = "Shows version")]
    Version,
}

#[derive(Debug, StructOpt)]
pub enum BacklightCommand {
    #[structopt(about = "Gets current keyboard backlight brightness value")]
    Get,

    #[structopt(about = "Gets current keyboard backlight brightness percentage value")]
    GetPercentage,

    #[structopt(about = "Sets backlight brightness as value")]
    Set { value: u64 },

    #[structopt(about = "Sets backlight brightness as percentage value")]
    SetPercentage { percentage_value: u64 },

    #[structopt(about = "Increases backlight brightness by percentage value")]
    Up {
        #[structopt(default_value = "5")]
        percentage_value: u64,
    },

    #[structopt(about = "Decreases backlight brightness by percentage value")]
    Down {
        #[structopt(default_value = "5")]
        percentage_value: u64,
    },

    #[structopt(about = "Sets backlight brightness as max")]
    Max,

    #[structopt(about = "Turns off backlight")]
    Off,

    #[structopt(about = "Performs breathing light mode")]
    BreathingLight {
        #[structopt(long, about = "percentage per step")]
        step: u64,

        #[structopt(long, about = "delay millisecond")]
        delay: u64,
    },
}

impl Command {
    #[inline]
    pub fn new() -> Self { Self::from_args() }

    pub fn run(self) {
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let exit_code = runtime.block_on(async move {
            match self {
                Self::ScreenBacklight { cmd } => cmd.screen().await,
                Self::KeyboardBacklight { cmd } => cmd.keyboard().await,
                Self::Completions { shell } => {
                    let mut app = Self::clap();
                    let binary_name = app.get_name().to_owned();
                    app.gen_completions_to(&binary_name, shell, &mut std::io::stdout());
                    EXIT_SUCCESS
                }
                Self::Version => Self::clap()
                    .write_version(&mut std::io::stdout())
                    .map(|_ret| EXIT_SUCCESS)
                    .unwrap_or(EXIT_FAILURE),
            }
        });

        std::process::exit(exit_code);
    }
}

impl BacklightCommand {
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
