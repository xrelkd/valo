use std::path::{Path, PathBuf};

use lazy_static::lazy_static;

use crate::backlight_controller::{Backlight, Device, Error};

lazy_static! {
    static ref BL_PATH: PathBuf = PathBuf::from("/sys/class/leds/smc::kbd_backlight");
    static ref BL_VALUE_FILE: PathBuf = BL_PATH.join("brightness");
    static ref BL_MAX_VALUE_FILE: PathBuf = BL_PATH.join("max_brightness");
}

#[derive(Debug, Default)]
pub struct KeyboardBacklight {
    device: Device,
}

impl KeyboardBacklight {
    pub async fn new() -> Result<Self, Error> { Self::load().await }

    async fn load() -> Result<Self, Error> {
        let device = Device::load(BL_VALUE_FILE.as_path(), BL_MAX_VALUE_FILE.as_path()).await?;
        Ok(Self { device })
    }
}

impl Backlight for KeyboardBacklight {
    fn max_value(&self) -> u64 { self.device.max_value() }

    fn current_value(&self) -> u64 { self.device.current_value() }

    fn current_value_file_path(&self) -> &Path { BL_VALUE_FILE.as_path() }

    fn maximum_value_file_path(&self) -> &Path { BL_MAX_VALUE_FILE.as_path() }

    async fn reload(&mut self) -> Result<(), Error> { Self::load().await.map(|s| *self = s) }
}
