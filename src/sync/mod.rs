mod sync_read;
mod sync_write;
use self::sync_write::{auto_write, write_only_new};
use super::{add_path, Error, QFilePath};
use crate::{directory_create, file};
use sync_read::read;
pub trait TraitQFileSync {
    fn read(&mut self) -> Result<String, Box<dyn Error>>;
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>>;
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>>;
    fn file(&mut self) -> Result<std::fs::File, Box<dyn Error>>;
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
}
impl TraitQFileSync for QFilePath {
    fn read(&mut self) -> Result<String, Box<dyn Error>> {
        read(self)
    }
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        add_path(path_file)
    }
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>> {
        directory_create(self)
    }
    fn file(&mut self) -> Result<std::fs::File, Box<dyn Error>> {
        file(self)
    }
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        auto_write(self, text)
    }
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        write_only_new(self, text)
    }
}
