use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not create tokio runtime, error: {source}"))]
    InitializeTokioRuntime { source: std::io::Error },

    #[snafu(display("Could not create Unix signal listener, error: {source}"))]
    CreateUnixSignalListener { source: std::io::Error },

    #[snafu(display("{source}"))]
    BacklightControl { source: crate::backlight_controller::Error },
}

impl From<crate::backlight_controller::Error> for Error {
    fn from(source: crate::backlight_controller::Error) -> Self {
        Self::BacklightControl { source }
    }
}
