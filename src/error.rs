/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum AppError {}
impl std::error::Error for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
