use std::path::PathBuf;

pub type IoTaskResult<T> = std::result::Result<T, IoTaskError>;

#[derive(Debug, Clone)]
pub enum IoTaskError {
    FileIo { path: PathBuf, error: String },
    Io(String),
    Toml(String),
    Yml(String),
}

impl std::error::Error for IoTaskError {}

impl std::fmt::Display for IoTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for IoTaskError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<toml::de::Error> for IoTaskError {
    fn from(value: toml::de::Error) -> Self {
        Self::Toml(value.to_string())
    }
}

impl From<serde_yml::Error> for IoTaskError {
    fn from(value: serde_yml::Error) -> Self {
        Self::Yml(value.to_string())
    }
}
