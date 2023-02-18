use crate::init::{constructor::add_path, work_with_elements::file};
use crate::{QFilePath, QPackError};
use std::fs;
pub trait QTratiSync {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError>;
    fn file(slf: &mut QFilePath) -> Result<fs::File, QPackError>;
}
impl QTratiSync for QFilePath {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError> {
        add_path(path_file)
    }
    fn file(slf: &mut QFilePath) -> Result<fs::File, QPackError> {
        file(slf)
    }
}
