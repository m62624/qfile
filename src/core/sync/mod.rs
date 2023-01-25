mod sync_read;
pub mod sync_trait;
mod sync_write;
use std::error::Error;
use std::path::PathBuf;

use crate::{QFilePath, QPackError};
pub fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
    if path_file.as_ref().to_string().is_empty() {
        return Err(Box::new(QPackError::PathIsEmpty));
    }
    let path_file = PathBuf::from(path_file.as_ref());
    if cfg!(unix) {
        if path_file.to_str().unwrap().contains("\\") {
            return Err(Box::new(QPackError::UnixPathIsIncorrect));
        }
    } else if cfg!(windows) {
        if path_file.to_str().unwrap().contains("/") {
            return Err(Box::new(QPackError::UnixPathIsIncorrect));
        }
    } else {
        return Err(Box::new(QPackError::SystemNotDefined));
    }
    Ok(QFilePath {
        context: super::CodeStatus::SyncCode(super::SyncPack {
            request_items: Default::default(),
            user_path: path_file,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: super::Flag::Auto,
            update_path: false,
        }),
    })
}
