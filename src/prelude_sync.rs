use crate::init::constructor::add_path;
use crate::{QFilePath, QPackError};
pub trait QTratiSync {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError>;
}
impl QTratiSync for QFilePath{
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError> {
        add_path(path_file)
    }
}