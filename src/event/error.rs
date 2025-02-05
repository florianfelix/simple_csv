use std::path::PathBuf;

pub type ActionResult<T> = std::result::Result<T, ActionError>;

#[derive(Debug, Clone)]
pub enum ActionError {
    FileIo { path: PathBuf, error: String },
}

impl std::error::Error for ActionError {}

impl std::fmt::Display for ActionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
