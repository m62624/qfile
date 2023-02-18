use crate::init::{
    constructor::add_path,
    work_with_elements::{file, folder_create},
};
use crate::paths::get_path::{get_path_buf, get_path_string};
use crate::read::read;
use crate::CodeStatus;
use crate::{QFilePath, QPackError};
use std::{fs, path::PathBuf};
pub trait QTraitSync {
    //================================================================
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError>;
    fn file(slf: &mut QFilePath) -> Result<fs::File, QPackError>;
    fn folder_create(slf: &mut QFilePath) -> Result<(), QPackError>;
    //================================================================
    fn get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, QPackError>;
    fn get_path_string(slf: &mut QFilePath) -> Result<String, QPackError>;
    //================================================================
    fn read(slf: &mut QFilePath) -> Result<String, QPackError>;
}
impl QTraitSync for QFilePath {
    //================================================================
    fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, QPackError> {
        add_path(path_file)
    }
    fn file(slf: &mut QFilePath) -> Result<fs::File, QPackError> {
        QFilePath::check_status_code(&slf, CodeStatus::SyncStatus)?;
        file(slf)
    }
    fn folder_create(slf: &mut QFilePath) -> Result<(), QPackError> {
        QFilePath::check_status_code(&slf, CodeStatus::SyncStatus)?;
        folder_create(slf)
    }
    //================================================================
    fn get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, QPackError> {
        QFilePath::check_status_code(&slf, CodeStatus::SyncStatus)?;
        get_path_buf(slf)
    }
    fn get_path_string(slf: &mut QFilePath) -> Result<String, QPackError> {
        QFilePath::check_status_code(&slf, CodeStatus::SyncStatus)?;
        get_path_string(slf)
    }
    //================================================================
    fn read(slf: &mut QFilePath) -> Result<String, QPackError> {
        QFilePath::check_status_code(&slf, CodeStatus::SyncStatus)?;
        read(slf)
    }
}
