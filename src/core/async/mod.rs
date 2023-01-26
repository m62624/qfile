use super::{Arc, AsyncFS, AsyncPath, Flag, QFilePath, QPackError};
use crate::core::{
    r#async::{async_trait::QFileAsync, get_path::async_get_path_buf},
    AsyncPack, CodeStatus,
};
use async_std::stream::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
mod async_read;
pub mod async_trait;
pub mod async_write;
pub mod get_path;

impl QFilePath {
    async fn async_way_step_by_step(&mut self) {
        async fn first_slash(sl: &mut QFilePath) {
            let slf = sl.context.get_async_pack_mut();
            let temp = slf.user_path.display().to_string();
            if cfg!(unix) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^/|^\.\./|^\./").unwrap();
                }
                if !SL.is_match(&temp) {
                    slf.user_path =
                        AsyncPath::PathBuf::from(format!("./{}", slf.user_path.display()));
                }
            }
            if cfg!(windows) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap();
                }
                if !SL.is_match(&temp) {
                    slf.user_path =
                        AsyncPath::PathBuf::from(format!(".\\{}", slf.user_path.display()));
                }
            }
        }
        first_slash(self).await;
        let slf = self.context.get_async_pack_mut();
        slf.request_items = slf
            .user_path
            .ancestors()
            .map(|element| element.display().to_string())
            .collect();
        if slf.request_items.last().unwrap().eq("") {
            slf.request_items.pop();
            if let Some(value) = slf.request_items.last_mut() {
                if cfg!(unix) {
                    if value.eq(&mut ".") {
                        *value = String::from("./")
                    }
                    if value.eq(&mut "..") {
                        *value = String::from("../")
                    }
                }
                if cfg!(windows) {
                    if value.eq(&mut ".") {
                        *value = String::from(".\\")
                    }
                    if value.eq(&mut "..") {
                        *value = String::from("..\\")
                    }
                }
            }
        }
        slf.request_items.reverse();
    }
    async fn async_return_file(path: &str) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>> {
        match async_std::fs::File::open(path).await {
            Ok(file) => Ok(file),
            Err(err) => Err(Box::new(err)),
        }
    }

    async fn async_directory_contents(path: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        if let Ok(mut paths) = async_std::fs::read_dir(path).await {
            loop {
                if let Some(item) = paths.next().await {
                    if let Ok(items) = item {
                        files.push(items.path().display().to_string());
                    };
                } else {
                    break;
                }
            }
        }
        return files;
    }
    async fn async_path_create(
        self: &mut Self,
        err: async_std::io::ErrorKind,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match err {
            async_std::io::ErrorKind::NotFound => {
                let fullpath = QFilePath::async_get_path_buf(self).await?;
                let filename = fullpath.file_name().unwrap().to_str().unwrap();
                let path_without_file = fullpath.to_str().unwrap().rsplit_once(filename).unwrap().0;
                {
                    self.context.get_async_pack_mut().user_path =
                        AsyncPath::PathBuf::from(path_without_file);
                    self.context.get_async_pack_mut().update_path = true;
                    self.context.get_async_pack_mut().file_name =
                        AsyncPath::PathBuf::from(filename);
                    self.context.get_async_pack_mut().flag = Flag::New;
                }
                AsyncFS::DirBuilder::new()
                    .recursive(true)
                    .create(path_without_file)
                    .await?;
                Ok(())
            }
            _ => Err(Box::new(QPackError::AsyncIOError(
                super::custom_errors::AsyncIO::IO(Arc::new(err.into())),
            ))),
        }
    }
}

pub async fn async_directory_create(
    slf: &mut QFilePath,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(AsyncFS::DirBuilder::new()
        .recursive(true)
        .create(slf.async_get_path_buf().await?)
        .await?)
}
pub async fn async_get_file(
    slf: &mut QFilePath,
) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>> {
    let path = slf.async_get_path_string().await?;
    match QFilePath::async_return_file(&path).await {
        Ok(file) => return Ok(file),
        Err(err) => return Err(err),
    }
}

pub fn add_path_for_async<T: AsRef<str> + Send + Sync>(
    path_file: T,
) -> Result<Arc<async_trait::AsyncMutex<QFilePath>>, Box<dyn Error + Send + Sync>> {
    if path_file.as_ref().is_empty() {
        return Err(Box::new(QPackError::PathIsEmpty));
    }
    let path_file = AsyncPath::PathBuf::from(path_file.as_ref());
    if cfg!(unix) {
        if path_file.to_str().unwrap().contains("\\") {
            return Err(Box::new(QPackError::UnixPathIsIncorrect));
        }
    } else if cfg!(windows) {
        if path_file.to_str().unwrap().contains("/") {
            return Err(Box::new(QPackError::WindowsPathIsIncorrect));
        }
    } else {
        return Err(Box::new(QPackError::SystemNotDefined));
    }
    Ok(Arc::new(async_trait::AsyncMutex::new(QFilePath {
        context: CodeStatus::AsyncCode(AsyncPack {
            request_items: Default::default(),
            user_path: AsyncPath::PathBuf::from(path_file.to_owned()),
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Flag::Auto,
            update_path: false,
        }),
    })))
}
