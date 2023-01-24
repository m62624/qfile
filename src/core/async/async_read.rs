use super::QFilePath;
use crate::core::r#async::async_get_path_string;
use async_std::io::ReadExt;
use std::error::Error;
pub async fn async_read(slf: &mut QFilePath) -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut text = String::new();
    QFilePath::async_return_file(&async_get_path_string(slf).await?)
        .await?
        .read_to_string(&mut text)
        .await?;
    Ok(text)
}
