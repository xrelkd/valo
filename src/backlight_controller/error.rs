use std::path::PathBuf;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Could not read value file: {}, error: {source}", file_path.display()))]
    ReadValueFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not write value file: {}, error: {source}", file_path.display()))]
    WriteValueFile { file_path: PathBuf, source: std::io::Error },

    #[snafu(display("Could not parse value, error: {source}"))]
    ParseValue { source: std::num::ParseIntError },

    #[snafu(display("Could not read directory, error {source}"))]
    ReadDirectory { dir_path: PathBuf, source: std::io::Error },

    #[snafu(display("No such device: {device}"))]
    NoSuchDevice { device: String },
}
