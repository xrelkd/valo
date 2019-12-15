use structopt::{clap::Shell as ClapShell, StructOpt};

use crate::backlight_controller::{Backlight, KeyboardBacklight, ScreenBacklight};

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "screen", about = "A simple command that changes screen backlight level.")]
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

    #[structopt(name = "completions", about = "Generate tab-completion scripts for your shell")]
    Completions {
        #[structopt(subcommand, name = "shell")]
        shell: CompletionShell,
    },
}

#[derive(Debug, StructOpt)]
pub enum BacklightCommand {
    /// Get current keyboard backlight brightness value
    Get,

    /// Get current keyboard backlight brightness percentage value
    GetPercentage,

    /// Set backlight brightness as value
    Set { value: u64 },

    /// Set backlight brightness as percentage value
    SetPercentage { percentage_value: u64 },

    /// Increase backlight brightness by percentage value
    Up {
        #[structopt(default_value = "5")]
        percentage_value: u64,
    },

    /// Decrease backlight brightness by percentage value
    Down {
        #[structopt(default_value = "5")]
        percentage_value: u64,
    },

    /// Set backlight brightness as max
    Max,

    /// Turn off backlight
    Off,

    /// Perform breathing light mode
    BreathingLight {
        #[structopt(long, about = "percentage per step")]
        step: u64,

        #[structopt(long, about = "delay millisecond")]
        delay: u64,
    },
}

#[derive(Debug, StructOpt, Clone)]
pub enum CompletionShell {
    #[structopt(name = "bash")]
    /// Generate shell completion for Bash
    Bash,

    #[structopt(name = "zsh")]
    /// Generate shell completion for Zsh
    Zsh,

    #[structopt(name = "fish")]
    /// Generate shell completion for Fish
    Fish,

    #[structopt(name = "powershell")]
    /// Generate shell completion for PowerShell
    PowerShell,

    #[structopt(name = "elvish")]
    /// Generate shell completion for Elvish
    Elvish,
}

impl Command {
    pub fn run(self) {
        let mut runtime = tokio::runtime::Runtime::new().unwrap();

        let exit_code = runtime.block_on(async move {
            match self {
                Command::ScreenBacklight { cmd } => cmd.screen().await,
                Command::KeyboardBacklight { cmd } => cmd.keyboard().await,
                Command::Completions { shell } => Self::generate_completion(shell).await,
            }
        });

        std::process::exit(exit_code);
    }

    async fn generate_completion(shell: CompletionShell) -> i32 {
        let mut app = Self::clap();
        let binary_name = app.get_name().to_owned();
        app.gen_completions_to(&binary_name, shell.into(), &mut std::io::stdout());
        0
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
                        _ = backlight.run_breathing(step, delay, 0, 100).fuse() => {
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

impl Into<ClapShell> for CompletionShell {
    fn into(self) -> ClapShell {
        match self {
            CompletionShell::Bash => ClapShell::Bash,
            CompletionShell::Elvish => ClapShell::Elvish,
            CompletionShell::Fish => ClapShell::Fish,
            CompletionShell::PowerShell => ClapShell::PowerShell,
            CompletionShell::Zsh => ClapShell::Zsh,
        }
    }
}
