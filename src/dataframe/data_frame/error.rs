// use serde::{Deserialize, Serialize};

pub type FrameResult<T> = std::result::Result<T, FrameError>;

#[derive(Debug, Clone)]
pub enum FrameError {
    Panic(String),
    Csv(String),
    Utf8(String),
    Io(String),
    NotUniformColumnWidths,
}

impl std::error::Error for FrameError {}
impl std::fmt::Display for FrameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<csv::Error> for FrameError {
    fn from(value: csv::Error) -> Self {
        Self::Csv(value.to_string())
    }
}

impl From<std::string::FromUtf8Error> for FrameError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Utf8(value.to_string())
    }
}

impl From<std::io::Error> for FrameError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}
