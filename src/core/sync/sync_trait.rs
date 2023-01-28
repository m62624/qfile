use super::super::{sync::sync_find::find_paths, RootDirectory};
use super::get_path::get_path_buf;
use super::{add_path, get_file};
use crate::{
    core::sync::{
        directory_create,
        sync_read::read,
        sync_write::{auto_write, write_only_new},
    },
    QFilePath,
};
use std::error::Error;
use std::path::PathBuf;

pub trait QFileSync {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
    fn get_path_buf(self: &mut Self) -> Result<PathBuf, Box<dyn Error>>;
    fn get_path_string(&mut self) -> Result<String, Box<dyn Error>>;
    fn change_path<T: AsRef<str>>(self: &mut Self, path: T) -> Result<(), Box<dyn Error>>;
    fn read(&mut self) -> Result<String, Box<dyn Error>>;
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>>;
    fn get_file(slf: &mut QFilePath) -> Result<std::fs::File, Box<dyn Error>>;
    fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
        sender: std::sync::mpsc::Sender<Option<Vec<std::path::PathBuf>>>,
        place: RootDirectory<T>,
        file_name: T,
    ) -> Result<(), std::sync::mpsc::SendError<Option<Vec<std::path::PathBuf>>>>;
}
impl QFileSync for QFilePath {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        Ok(add_path(path_file)?)
    }
    fn get_path_buf(self: &mut Self) -> Result<PathBuf, Box<dyn Error>> {
        Ok(get_path_buf(self)?)
    }
    fn get_path_string(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(get_path_buf(self)?.to_str().unwrap().to_owned())
    }
    fn change_path<T: AsRef<str>>(self: &mut Self, path: T) -> Result<(), Box<dyn Error>> {
        Ok({
            self.context.get_sync_pack_mut().user_path = PathBuf::from(path.as_ref());
            self.context.get_sync_pack_mut().correct_path = Default::default();
            self.context.get_sync_pack_mut().request_items.clear();
        })
    }
    fn read(&mut self) -> Result<String, Box<dyn Error>> {
        Ok(read(self)?)
    }
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        Ok(auto_write(self, text)?)
    }
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        Ok(write_only_new(self, text)?)
    }
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(directory_create(self)?)
    }
    fn get_file(slf: &mut QFilePath) -> Result<std::fs::File, Box<dyn Error>> {
        Ok(get_file(slf)?)
    }
    fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
        sender: std::sync::mpsc::Sender<Option<Vec<std::path::PathBuf>>>,
        place: RootDirectory<T>,
        file_name: T,
    ) -> Result<(), std::sync::mpsc::SendError<Option<Vec<std::path::PathBuf>>>> {
        find_paths(sender, place, file_name)
    }
}
