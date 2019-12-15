use crate::backlight_controller::{Backlight, KeyboardBacklight, ScreenBacklight};

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(
        name = "screen",
        about = "A simple command that changes screen backlight level."
    )]
    ScreenBacklight {
        #[structopt(subcommand, name = "screen")]
        cmd: BacklightCommand,
    },
    #[structopt(
        name = "keyboard",
        about = "A simple command that changes MacBooks' keyboard backlight level."
    )]
    KeyboardBacklight {
        #[structopt(subcommand, name = "keyboard")]
        cmd: BacklightCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum BacklightCommand {
    #[structopt(about = "Get current keyboard backlight brightness value")]
    Get,

    #[structopt(about = "Get current keyboard backlight brightness percentage value")]
    GetPercentage,

    #[structopt(about = "Set backlight brightness as value")]
    Set { value: u64 },

    #[structopt(about = "Set backlight brightness as percentage value")]
    SetPercentage { percentage_value: u64 },

    #[structopt(about = "Increase backlight brightness by percentage value")]
    Up { percentage_value: u64 },

    #[structopt(about = "Decrease backlight brightness by percentage value")]
    Down { percentage_value: u64 },

    #[structopt(about = "Set backlight brightness as max")]
    Max,

    #[structopt(about = "Turn off backlight")]
    Off,

    BreathingLight {
        #[structopt(long)]
        step: u64,

        #[structopt(long)]
        delay: u64,
    },
}

impl Command {
    pub fn run(self) {
        let mut runtime = tokio::runtime::Runtime::new().unwrap();

        let exit_code = runtime.block_on(async move {
            match self {
                Command::ScreenBacklight { cmd } => cmd.screen().await,
                Command::KeyboardBacklight { cmd } => cmd.keyboard().await,
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
                eprintln!("{}", err);
                1
            }
        }
    }

    pub async fn screen(self) -> i32 {
        match ScreenBacklight::new().await {
            Ok(backlight) => self.run(backlight).await,
            Err(err) => {
                eprintln!("{}", err);
                1
            }
        }
    }

    async fn run<B: Backlight>(self, mut backlight: B) -> i32 {
        use BacklightCommand::*;
        let exit_code = match self {
            Get => {
                println!("{}", backlight.current_value());
                Ok(0)
            }
            GetPercentage => {
                println!("{}", backlight.current_percentage());
                Ok(0)
            }
            Set { value } => backlight.set(value).await,
            SetPercentage { percentage_value } => backlight.set_percentage(percentage_value).await,
            Up { percentage_value } => backlight.up(percentage_value).await,
            Down { percentage_value } => backlight.down(percentage_value).await,
            Off => backlight.off().await,
            Max => backlight.max().await,
            BreathingLight { step, delay } => {
                use futures_util::FutureExt;
                use tokio::signal::unix::{signal, SignalKind};

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
                        _ = backlight.run_breathing(step, delay).fuse() => {
                            break;
                        },
                        _ = signal_receiver.fuse() => {
                            break;
                        },
                    }
                }

                backlight.set(origin_value).await
            }
        };

        exit_code.map(|_value| 0).unwrap_or(1)
    }
}
