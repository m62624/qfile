use crate::find::pathfinder::find_paths;
use crate::init::{
    constructor::add_path,
    work_with_elements::{file, folder_create},
};
use crate::paths::get_path::{get_path_buf, get_path_string};
use crate::read::read::*;
use crate::write::write::{auto_write, write_only_new};
use crate::CodeStatus;
use crate::Directory;
use crate::{QFilePath, QPackError};
use std::sync::mpsc::{SendError, Sender};
use std::{fs, path::PathBuf};
/*
The prelude_sync module is a collection of frequently used items that are imported automatically when the QPack library is used. This module saves the user from having to import each item manually.
 */
pub trait QTraitSync {
    /// The add_path constructor from the qfile library allows you to create an object of type QFilePack that represents a file in a given path. (**Not case-sensitive**)
    /// To create the object you must pass the path to the file in string format.
    /// The path can be absolute or relative, and can also contain ... symbols to jump to a higher level in the folder hierarchy.
    /// # Example
    /// ```
    /// use qfile::{QFilePath, QPackError, QTraitSync};
    /// use std::error::Error;
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let file = QFilePath::add_path("my_folder/my_file.txt")?;
    ///     Ok(())
    /// }
    /// ```
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError>;
    fn file(&mut self) -> Result<fs::File, QPackError>;
    fn folder_create(&mut self) -> Result<(), QPackError>;
    //================================================================
    fn get_path_buf(&mut self) -> Result<PathBuf, QPackError>;
    fn get_path_string(&mut self) -> Result<String, QPackError>;
    //================================================================
    fn read(&mut self) -> Result<String, QPackError>;
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), QPackError>;
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), QPackError>;
}
impl QTraitSync for QFilePath {
    //================================================================
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError> {
        add_path(path_file)
    }
    fn file(&mut self) -> Result<fs::File, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        file(self)
    }
    fn folder_create(&mut self) -> Result<(), QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        folder_create(self)
    }
    //================================================================
    fn get_path_buf(&mut self) -> Result<PathBuf, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        get_path_buf(self)
    }
    fn get_path_string(&mut self) -> Result<String, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        get_path_string(self)
    }
    //================================================================
    fn read(&mut self) -> Result<String, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        read(self)
    }
    fn auto_write<T: AsRef<str>>(&mut self, text: T) -> Result<(), QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        auto_write(self, text)
    }
    fn write_only_new<T: AsRef<str>>(&mut self, text: T) -> Result<(), QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::SyncStatus)?;
        write_only_new(self, text)
    }
}
impl QFilePath {
    pub fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
        place: Directory<T>,
        names: Vec<T>,
        excluded_dirs: Option<Vec<T>>,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>> {
        find_paths(place, names, excluded_dirs, follow_link, sender)
    }
}
