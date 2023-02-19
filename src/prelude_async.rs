// use crate::find::pathfinder::find_paths;
use crate::init::{
    constructor::async_add_path,
    work_with_elements::{async_file, async_folder_create},
};
use crate::paths::get_path::{async_get_path_buf, async_get_path_string};
use crate::read::read::async_read;
use crate::write::write::{async_auto_write, async_write_only_new};
use crate::CodeStatus;
use crate::{QFilePath, QPackError};
use async_fs;
use async_trait::async_trait;
use std::error::Error;
use std::path::PathBuf;
/*
The prelude_async module is a collection of frequently used items that are imported automatically when the QPack library is used. This module saves the user from having to import each item manually.
 */
#[async_trait]
pub trait QTraitAsync {
    //================================================================
    async fn async_add_path<T: AsRef<str> + Send + Sync>(
        path_file: T,
    ) -> Result<QFilePath, QPackError>;
    async fn async_file(&mut self) -> Result<async_fs::File, QPackError>;
    async fn async_folder_create(&mut self) -> Result<(), QPackError>;
    //================================================================
    async fn async_get_path_buf(&mut self) -> Result<PathBuf, QPackError>;
    async fn async_get_path_string(&mut self) -> Result<String, QPackError>;
    //================================================================
    async fn async_read(&mut self) -> Result<String, QPackError>;
    async fn async_auto_write<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_write_only_new<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
#[async_trait]
impl QTraitAsync for QFilePath {
    //================================================================
    async fn async_add_path<T: AsRef<str> + Send + Sync>(
        path_file: T,
    ) -> Result<QFilePath, QPackError> {
        async_add_path(path_file).await
    }
    async fn async_file(&mut self) -> Result<async_fs::File, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_file(self).await
    }
    async fn async_folder_create(&mut self) -> Result<(), QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_folder_create(self).await
    }
    //================================================================
    async fn async_get_path_buf(&mut self) -> Result<PathBuf, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_get_path_buf(self).await
    }
    async fn async_get_path_string(&mut self) -> Result<String, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_get_path_string(self).await
    }
    //================================================================
    async fn async_read(&mut self) -> Result<String, QPackError> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_read(self).await
    }
    async fn async_auto_write<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_auto_write(self, text).await
    }
    async fn async_write_only_new<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        QFilePath::check_status_code(&self, CodeStatus::AsyncStatus)?;
        async_write_only_new(self, text).await
    }
}
