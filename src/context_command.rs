use super::Directory;
use super::QFilePath;
use crate::init::work_with_elements::async_return_file;
use crate::init::work_with_elements::return_file;
use crate::prelude_async::QTraitAsync;
use crate::QTraitSync;
use futures_lite::AsyncReadExt;
use std::error::Error;
use std::io::Read;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
pub mod alias_context {

    use super::*;
    #[derive(Debug)]
    pub enum Action<'a, T: AsRef<str> + Sync + Send> {
        Find(Directory<T>, Vec<T>, Option<Vec<T>>, bool, Sender<PathBuf>),
        Read(&'a mut String),
        Write(T),
        WriteNew(T),
    }
    pub fn action_on_file<T: AsRef<str> + Send + Sync>(
        slf: &mut QFilePath,
        action: Action<T>,
    ) -> Result<(), Box<dyn Error>> {
        match action {
            Action::Find(place, names, excluded_dirs, follow_link, sender) => {
                QFilePath::find_paths(place, names, excluded_dirs, follow_link, sender)?;
            }
            Action::Read(text) => {
                return_file(&slf.get_path_string()?)?.read_to_string(text)?;
            }
            Action::Write(text) => {
                slf.auto_write(text)?;
            }
            Action::WriteNew(text) => {
                slf.write_only_new(text)?;
            }
        }
        Ok(())
    }
    pub async fn async_action_on_file<T: AsRef<str> + Send + Sync>(
        slf: &mut QFilePath,
        action: Action<'_, T>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match action {
            Action::Find(place, names, excluded_dirs, follow_link, sender) => {
                QFilePath::find_paths(place, names, excluded_dirs, follow_link, sender)?;
            }
            Action::Read(text) => {
                async_return_file(&slf.get_path_string()?)
                    .await?
                    .read_to_string(text)
                    .await?;
            }
            Action::Write(text) => {
                slf.async_auto_write(text).await?;
            }
            Action::WriteNew(text) => {
                slf.async_write_only_new(text).await?;
            }
        }
        Ok(())
    }
}
