use super::{QFilePath, QPackError};
use crate::init::work_with_elements::*;
use crate::paths::get_path::*;
use futures_lite::AsyncReadExt;
use std::io::Read;
pub fn read(slf: &mut QFilePath) -> Result<String, QPackError> {
    let mut text = String::new();
    return_file(&get_path_string(slf)?)?.read_to_string(&mut text)?;
    Ok(text)
}
pub async fn async_read(slf: &mut QFilePath) -> Result<String, QPackError> {
    let mut text = String::new();
    async_return_file(&async_get_path_string(slf).await?)
        .await?
        .read_to_string(&mut text)
        .await?;
    Ok(text)
}
