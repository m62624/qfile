use super::add_path;
use super::get_path::get_path_buf;
use crate::QFilePath;
use std::error::Error;
use std::path::PathBuf;
pub trait SyncQPack {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn get_path_buf(self: &mut Self) -> Result<PathBuf, Box<dyn Error>> {
        todo!()
    }
    fn get_path_string(&mut self) -> Result<String, Box<dyn Error>> {
        todo!()
    }
    fn change_path<T: AsRef<str>>(self: &mut Self, path: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn read(&mut self) -> Result<String, Box<dyn Error>> {
        todo!()
    }
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        todo!()
    }
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
impl SyncQPack for QFilePath {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        Ok(add_path(path_file)?)
    }
    fn get_path_buf(self: &mut Self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(get_path_buf(self)?)
    }
}
