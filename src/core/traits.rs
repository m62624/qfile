use super::{OptionCodePathBuf, QFilePath, QPathError};
use async_trait::async_trait;
use std::error::Error;
pub trait PathPattern {
    fn init_user_path<T: AsRef<str>>(path_file: T) -> Result<OptionCodePathBuf, Box<dyn Error>>;
    fn correct_path(&mut self);
}
#[async_trait]
pub trait PathPatternAsync {
    async fn async_init_user_path<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<OptionCodePathBuf, Box<dyn Error>>;
    async fn async_correct_path(&mut self);
}

impl<'a> PathPattern for QFilePath<'a> {
    fn init_user_path<T: AsRef<str>>(path_file: T) -> Result<OptionCodePathBuf, Box<dyn Error>> {
        if path_file.as_ref().is_empty() {
            return Err(Box::new(QPathError::PathIsEmpty));
        }
        if cfg!(target_family = "unix") {
            if path_file.as_ref().contains("\\") {
                return Err(Box::new(QPathError::UnixPathIsIncorrect));
            }
        } else if cfg!(target_family = "windows") {
            if path_file.as_ref().contains("/") {
                return Err(Box::new(QPathError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPathError::SystemNotDefined));
        }
        return Ok(OptionCodePathBuf::SCPathBuf(std::path::PathBuf::from(
            path_file.as_ref(),
        )));
    }
}
#[async_trait]
impl<'a> PathPatternAsync for QFilePath<'a> {
    async fn async_init_user_path<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<OptionCodePathBuf, Box<dyn Error>> {
        if path_file.as_ref().is_empty() {
            return Err(Box::new(QPathError::PathIsEmpty));
        }
        if cfg!(target_family = "unix") {
            if path_file.as_ref().contains("\\") {
                return Err(Box::new(QPathError::UnixPathIsIncorrect));
            }
        } else if cfg!(target_family = "windows") {
            if path_file.as_ref().contains("/") {
                return Err(Box::new(QPathError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPathError::SystemNotDefined));
        }
        return Ok(OptionCodePathBuf::ACPathBuf(
            async_std::path::PathBuf::from(path_file.as_ref()),
        ));
    }
}
