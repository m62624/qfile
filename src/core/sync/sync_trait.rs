use std::error::Error;
use std::path::PathBuf;

use crate::QFilePath;
pub trait SyncQPack {
    fn add_path_for_async<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>>;
    fn async_get_path_buf(self: &mut Self) -> Result<PathBuf, Box<dyn Error>>;
    fn async_get_path_string(&mut self) -> Result<String, Box<dyn Error>>;
    fn async_change_path<T: AsRef<str>>(self: &mut Self, path: T) -> Result<(), Box<dyn Error>>;
    fn async_read(&mut self) -> Result<String, Box<dyn Error>>;
    fn async_auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn async_write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn async_directory_create(&mut self) -> Result<(), Box<dyn Error>>;
}
