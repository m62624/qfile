// use crate::find::pathfinder::find_paths;
use crate::init::{
    constructor::add_path,
    work_with_elements::{file, folder_create},
};
use crate::paths::get_path::{get_path_buf, get_path_string};
use crate::read::read;
use crate::write::write::{auto_write, write_only_new};
use crate::CodeStatus;
// use crate::Directory;
use crate::{QFilePath, QPackError};
// use std::sync::mpsc::{SendError, Sender};
use std::{fs, path::PathBuf};
pub trait QTraitSync {
    //================================================================
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
