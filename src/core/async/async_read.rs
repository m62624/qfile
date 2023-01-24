use async_std::io::ReadExt;

use super::QFilePath;
use std::error::Error;
impl QFilePath {
    pub async fn async_read(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let mut text = String::new();
        QFilePath::async_return_file(&self.async_get_path_string().await?)
            .await?
            .read_to_string(&mut text)
            .await?;
        Ok(text)
    }
}
