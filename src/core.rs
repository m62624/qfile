use super::QError::QPackError;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub enum Flag {
    Old,
    Auto,
    New,
}
#[derive(Debug, Clone)]
pub struct QFilePath {
    request_items: Vec<String>,
    user_path: PathBuf,
    file_name: PathBuf,
    correct_path: PathBuf,
    flag: Flag,
    update_path: bool,
}

impl Drop for QFilePath {
    fn drop(&mut self) {}
}
