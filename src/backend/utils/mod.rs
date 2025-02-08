use std::path::PathBuf;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use super::IoCommandResult;

pub async fn read_file(path: &PathBuf) -> IoCommandResult<String> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
pub async fn save_file(path: &PathBuf, content: &str) -> IoCommandResult<()> {
    let data: &[u8] = content.as_bytes();
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(data).await?;
    Ok(())
}
