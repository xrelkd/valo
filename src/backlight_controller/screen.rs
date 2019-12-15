use crate::backlight_controller::{Backlight, Device, Error};

use std::path::{Path, PathBuf};

lazy_static! {
    static ref BL_PATH: PathBuf = PathBuf::from("/sys/class/backlight");
}

#[derive(Debug, Default)]
pub struct ScreenBacklight {
    device: Device,
    best_controller: String,
    value_file_path: PathBuf,
    maximum_value_file_path: PathBuf,
}

impl ScreenBacklight {
    pub async fn new() -> Result<Self, Error> {
        Self::select_best_device().await
    }

    async fn select_best_device() -> Result<ScreenBacklight, Error> {
        use tokio::fs;
        let mut backlight_dir = fs::read_dir(BL_PATH.as_path()).await?;
        let mut best_value: u64 = 0;
        let mut best_controller = None;

        while let Some(current_dir) = backlight_dir.next_entry().await? {
            let current_value =
                tokio::fs::read_to_string(current_dir.path().join("max_brightness").as_path())
                    .await
                    .unwrap_or("0".to_owned())
                    .trim()
                    .parse::<u64>()
                    .unwrap_or(0);

            if current_value > best_value {
                best_value = current_value;
                best_controller = Some(current_dir.file_name());
            }
        }

        match best_controller {
            Some(best_controller) => {
                let best_dir = BL_PATH.join(&best_controller);
                let maximum_value_file_path = best_dir.join("max_brightness");
                let value_file_path = best_dir.join("brightness");
                let device =
                    Device::load(value_file_path.as_path(), maximum_value_file_path.as_path())
                        .await?;
                Ok(ScreenBacklight {
                    device,
                    value_file_path,
                    maximum_value_file_path,
                    best_controller: best_controller.to_str().unwrap().to_owned(),
                })
            }
            None => Err(Error::NoSuchDevice("screen light".to_owned())),
        }
    }
}

#[async_trait]
impl Backlight for ScreenBacklight {
    fn max_value(&self) -> u64 {
        self.device.max_value()
    }

    fn current_value(&self) -> u64 {
        self.device.current_value()
    }

    fn current_value_file_path(&self) -> &Path {
        self.value_file_path.as_path()
    }

    fn maximum_value_file_path(&self) -> &Path {
        self.maximum_value_file_path.as_path()
    }

    async fn reload(&mut self) -> Result<(), Error> {
        let device =
            Device::load(self.current_value_file_path(), self.maximum_value_file_path()).await?;

        self.device = device;
        Ok(())
    }
}
