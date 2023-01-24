pub use self::async_trait::AsyncQPackTrait;
use super::{AsyncFS, AsyncPath, QFilePath, QPackError};
use async_std::stream::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
pub mod async_read;
pub mod async_trait;
pub mod async_write;
use super::Flag;
impl QFilePath {
    pub async fn async_get_path_string(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(QFilePath::async_get_path_buf(self)
            .await?
            .to_str()
            .unwrap()
            .to_owned())
    }
    pub async fn async_directory_create(
        self: &mut Self,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(AsyncFS::DirBuilder::new()
            .recursive(true)
            .create(self.async_get_path_buf().await?)
            .await?)
    }
}
impl QFilePath {
    async fn async_way_step_by_step(&mut self) {
        async fn first_slash(sl: &mut QFilePath) {
            let slf = sl.Context.get_pack_mut();
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
        let slf = self.Context.get_pack_mut();
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
}

impl QFilePath {
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
    async fn async_dir_create(
        self: &mut Self,
        err: async_std::io::ErrorKind,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        match err {
            async_std::io::ErrorKind::NotFound => {
                let fullpath = QFilePath::async_get_path_buf(self).await?;
                let filename = fullpath.file_name().unwrap().to_str().unwrap();
                let path_without_file = fullpath.to_str().unwrap().rsplit_once(filename).unwrap().0;
                {
                    self.Context.get_pack_mut().user_path =
                        AsyncPath::PathBuf::from(path_without_file);
                    self.Context.get_pack_mut().update_path = true;
                    self.Context.get_pack_mut().file_name = AsyncPath::PathBuf::from(filename);
                    self.Context.get_pack_mut().flag = Flag::New;
                }
                AsyncFS::DirBuilder::new()
                    .recursive(true)
                    .create(path_without_file)
                    .await?;
                Ok(())
            }
            _ => Err(Box::new(QPackError::AsyncIOError(
                super::custom_errors::AsyncIO::IO(err.into()),
            ))),
        }
    }
}