use std::path::PathBuf;

pub type IoTaskResult<T> = std::result::Result<T, IoTaskError>;

#[derive(Debug, Clone)]
pub enum IoTaskError {
    FileIo { path: PathBuf, error: String },
}

impl std::error::Error for IoTaskError {}

impl std::fmt::Display for IoTaskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
