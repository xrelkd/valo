use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

mod keyboard;
mod screen;

pub use self::keyboard::KeyboardBacklight;
pub use self::screen::ScreenBacklight;

#[derive(Debug)]
pub struct Device {
    current_value: AtomicU64,
    max_value: AtomicU64,
}

impl Default for Device {
    fn default() -> Device {
        Device {
            current_value: AtomicU64::new(0),
            max_value: AtomicU64::new(255),
        }
    }
}

impl Device {
    pub async fn load(value_file_path: &Path, max_value_file_path: &Path) -> Result<Device, Error> {
        use tokio::fs;
        let current_value = AtomicU64::new(
            fs::read_to_string(value_file_path)
                .await?
                .trim()
                .parse::<u64>()?,
        );

        let max_value = AtomicU64::new(
            fs::read_to_string(max_value_file_path)
                .await
                .unwrap_or(String::from("255"))
                .trim()
                .parse::<u64>()?,
        );

        Ok(Device {
            current_value,
            max_value,
        })
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

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    StdIo(std::io::Error),

    #[fail(display = "failed parse: {}", _0)]
    ParseIntError(std::num::ParseIntError),

    #[fail(display = "no such device: {}", _0)]
    NoSuchDevice(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::StdIo(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

#[async_trait]
pub trait Backlight: Send + Sync {
    fn max_value(&self) -> u64;

    fn current_value(&self) -> u64;

    fn current_percentage(&self) -> u64 {
        (100 * self.current_value() / self.max_value())
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

        // let _ = tokio::fs::write(self.current_value_file_path(), format!("{}", next)).await?;
        let _ = std::fs::write(self.current_value_file_path(), format!("{}", next));
        self.reload().await?;
        Ok(self.current_value())
    }

    async fn up(&mut self, percentage_value: u64) -> Result<u64, Error> {
        self.change_value(BacklightAction::Up { percentage_value })
            .await
    }

    async fn down(&mut self, percentage_value: u64) -> Result<u64, Error> {
        self.change_value(BacklightAction::Down { percentage_value })
            .await
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

    async fn run_breathing(&mut self, step: u64, delay: u64) {
        use std::time::Duration;
        use tokio::time;

        let step = if step == 0 || step > 100 { 5 } else { step };
        let delay = if delay == 0 { 100 } else { delay };
        let iteration = 100 / step;
        loop {
            for _ in 1..iteration {
                let _ = self.up(step).await;
                time::delay_for(Duration::from_millis(delay)).await;
            }

            let _ = self.max().await;
            time::delay_for(Duration::from_millis(delay)).await;

            for _ in 1..iteration {
                let _ = self.down(step).await;
                time::delay_for(Duration::from_millis(delay)).await;
            }

            let _ = self.off().await;
            time::delay_for(Duration::from_millis(delay)).await;
        }
    }

    fn compute_value(&self, percentage_value: u64) -> u64 {
        (percentage_value as f64 * 0.01 * self.max_value() as f64) as u64
    }
}
