use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use snafu::{ResultExt, Snafu};

mod keyboard;
mod screen;

pub use self::{keyboard::KeyboardBacklight, screen::ScreenBacklight};

#[derive(Debug)]
pub struct Device {
    current_value: AtomicU64,
    max_value: AtomicU64,
}

impl Default for Device {
    fn default() -> Device {
        Device { current_value: AtomicU64::new(0), max_value: AtomicU64::new(255) }
    }
}

impl Device {
    pub async fn load(value_file_path: &Path, max_value_file_path: &Path) -> Result<Device, Error> {
        use tokio::fs;
        let current_value = AtomicU64::new(
            fs::read_to_string(value_file_path)
                .await
                .context(ReadValueFile { file_path: value_file_path.to_owned() })?
                .trim()
                .parse::<u64>()
                .context(ParseValue)?,
        );

        let max_value = AtomicU64::new(
            fs::read_to_string(max_value_file_path)
                .await
                .unwrap_or(String::from("255"))
                .trim()
                .parse::<u64>()
                .context(ParseValue)?,
        );

        Ok(Device { current_value, max_value })
    }

    pub fn max_value(&self) -> u64 {
        self.max_value.load(Ordering::Relaxed)
    }

    pub fn current_value(&self) -> u64 {
        self.current_value.load(Ordering::Relaxed)
    }
}

pub enum BacklightAction {
    Up { percentage_value: u64 },
    Down { percentage_value: u64 },
    Set { value: u64 },
    Max,
    Off,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read value file: {}, error: {}", file_path.display(), source))]
    ReadValueFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not write value file: {}, error: {}", file_path.display(), source))]
    WriteValueFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not parse value, error: {}", source))]
    ParseValue { source: std::num::ParseIntError },

    #[snafu(display("Could not read directory, error {}", source))]
    ReadDir { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("No such device: {}", device))]
    NoSuchDevice { device: String },
}

#[async_trait]
pub trait Backlight: Send + Sync {
    fn max_value(&self) -> u64;

    fn current_value(&self) -> u64;

    fn current_percentage(&self) -> u64 {
        100 * self.current_value() / self.max_value()
    }

    fn current_value_file_path(&self) -> &Path;

    fn maximum_value_file_path(&self) -> &Path;

    async fn reload(&mut self) -> Result<(), Error>;

    async fn change_value(&mut self, action: BacklightAction) -> Result<u64, Error> {
        let current_value = self.current_value();
        let max_value = self.max_value();

        use BacklightAction::*;
        let next = match action {
            Up { percentage_value } => {
                let value = self.compute_value(percentage_value);
                if current_value >= max_value - value {
                    max_value
                } else {
                    current_value + value
                }
            }
            Down { percentage_value } => {
                let value = self.compute_value(percentage_value);
                if current_value <= value {
                    0
                } else {
                    current_value - value
                }
            }
            Set { value } => std::cmp::min(value, max_value),
            Max => max_value,
            Off => 0,
        };

        tokio::fs::write(self.current_value_file_path(), format!("{}", next))
            .await
            .context(WriteValueFile { file_path: self.current_value_file_path().to_owned() })?;
        self.reload().await?;
        Ok(self.current_value())
    }

    async fn up(&mut self, percentage_value: u64) -> Result<u64, Error> {
        self.change_value(BacklightAction::Up { percentage_value }).await
    }

    async fn down(&mut self, percentage_value: u64) -> Result<u64, Error> {
        self.change_value(BacklightAction::Down { percentage_value }).await
    }

    async fn max(&mut self) -> Result<u64, Error> {
        self.change_value(BacklightAction::Max).await
    }

    async fn off(&mut self) -> Result<u64, Error> {
        self.change_value(BacklightAction::Off).await
    }

    async fn set(&mut self, value: u64) -> Result<u64, Error> {
        self.change_value(BacklightAction::Set { value }).await
    }

    async fn set_percentage(&mut self, value: u64) -> Result<u64, Error> {
        let value = self.compute_value(value);
        self.change_value(BacklightAction::Set { value }).await
    }

    async fn run_breathing(
        &mut self,
        step: u64,
        delay: u64,
        min_percentage: u64,
        max_percentage: u64,
    ) {
        use std::time::Duration;
        use tokio::time;

        let step = if step == 0 || step > 100 { 5 } else { step };
        let delay = if delay == 0 { 100 } else { delay };
        let iteration = 100 / step;

        let (min_percentage, max_percentage) = {
            let fix = |value| if value > 100 { 100 } else { value };
            (fix(min_percentage), fix(max_percentage))
        };

        loop {
            for _ in 1..iteration {
                let _ = self.up(step).await;
                time::delay_for(Duration::from_millis(delay)).await;
            }

            let _ = self.set_percentage(max_percentage).await;
            time::delay_for(Duration::from_millis(delay)).await;

            for _ in 1..iteration {
                let _ = self.down(step).await;
                time::delay_for(Duration::from_millis(delay)).await;
            }

            let _ = self.set_percentage(min_percentage).await;
            time::delay_for(Duration::from_millis(delay)).await;
        }
    }

    fn compute_value(&self, percentage_value: u64) -> u64 {
        (percentage_value as f64 * 0.01 * self.max_value() as f64) as u64
    }
}
