mod sync_find;
mod sync_read;
mod sync_write;
use self::{
    sync_find::{find_paths, find_paths_regex},
    sync_write::{auto_write, write_only_new},
};
use super::CodeStatus;
use super::{Directory, Error, QFilePath};
use crate::get_path::{get_path_buf, get_path_string};
use crossbeam::channel::{SendError, Sender};
use regex::Regex;
use std::path::PathBuf;
use sync_read::read;
pub trait TraitQFileSync {
    fn get_path_buf(&mut self) -> Result<PathBuf, Box<dyn Error>>;
    fn get_path_string(&mut self) -> Result<String, Box<dyn Error>>;
    fn read(&mut self) -> Result<String, Box<dyn Error>>;
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>>;
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>>;
    fn file(&mut self) -> Result<std::fs::File, Box<dyn Error>>;
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>>;
    fn find_paths<T: AsRef<str> + Send + Sync + Copy + 'static>(
        place: Directory,
        name: T,
        follow_link: bool,
        sender: Sender<std::path::PathBuf>,
    ) -> Result<(), SendError<std::path::PathBuf>>;
    fn find_paths_regex(
        place: Directory,
        pattern: Regex,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>>;
}
impl TraitQFileSync for QFilePath {
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        QFilePath::add_path(path_file)
    }
    fn get_path_buf(&mut self) -> Result<PathBuf, Box<dyn Error>> {
        get_path_buf(self)
    }
    fn get_path_string(&mut self) -> Result<String, Box<dyn Error>> {
        get_path_string(self)
    }
    fn directory_create(&mut self) -> Result<(), Box<dyn Error>> {
        self.check_status_code(CodeStatus::SyncStatus)?;
        QFilePath::directory_create(self)
    }
    fn file(&mut self) -> Result<std::fs::File, Box<dyn Error>> {
        QFilePath::file(self)
    }
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        self.check_status_code(CodeStatus::SyncStatus)?;
        auto_write(self, text)
    }
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), Box<dyn Error>> {
        self.check_status_code(CodeStatus::SyncStatus)?;
        write_only_new(self, text)
    }
    fn read(&mut self) -> Result<String, Box<dyn Error>> {
        self.check_status_code(CodeStatus::SyncStatus)?;
        read(self)
    }
    fn find_paths<T: AsRef<str> + Send + Sync + Copy + 'static>(
        place: Directory,
        name: T,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>> {
        find_paths(place, name, follow_link, sender)
    }
    fn find_paths_regex(
        place: Directory,
        pattern: Regex,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>> {
        find_paths_regex(place, pattern, follow_link, sender)
    }
}
