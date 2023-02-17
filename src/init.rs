use super::{CodeStatus, Flag, PathBuf, QFilePath, QPackError};
use std::error::Error;
mod constructor {
    use super::*;
    fn core<T: AsRef<str>>(path_file: T, status: CodeStatus) -> Result<QFilePath, Box<dyn Error>> {
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
                return Err(Box::new(QPackError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPackError::SystemNotDefined));
        }
        Ok(QFilePath {
            request_items: Default::default(),
            user_path: path_file,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Flag::Auto,
            update_path: false,
            status,
        })
    }
    pub fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        core(path_file, CodeStatus::SyncStatus)
    }
    pub async fn async_add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        core(path_file, CodeStatus::AsyncStatus)
    }
}
