use std::path::PathBuf;

use dataframe::FrameError;

pub type IoCommandResult<T> = std::result::Result<T, IoCommandError>;

#[derive(Debug, Clone)]
pub enum IoCommandError {
    FileIo { path: PathBuf, error: String },
    Io(String),
    Toml(String),
    Yml(String),
    DataFrame(FrameError),
}

impl std::error::Error for IoCommandError {}

impl std::fmt::Display for IoCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for IoCommandError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<toml::de::Error> for IoCommandError {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value.to_string())
    }
}

impl From<serde_yml::Error> for IoCommandError {
    fn from(value: serde_yml::Error) -> Self {
        Self::Yml(value.to_string())
    }
}

impl From<FrameError> for IoCommandError {
    fn from(value: FrameError) -> Self {
        Self::DataFrame(value)
    }
}
