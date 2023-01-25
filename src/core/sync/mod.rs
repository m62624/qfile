mod sync_read;
pub mod sync_trait;
mod sync_write;
use super::custom_errors::SyncIO;
use crate::{
    core::{sync::sync_trait::QFileSync, Flag},
    QFilePath, QPackError,
};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    error::Error,
    fs::{self, File},
    path::PathBuf,
};
pub mod get_path;
impl QFilePath {
    fn way_step_by_step(&mut self) {
        fn first_slash(sl: &mut QFilePath) {
            let slf = sl.context.get_sync_pack_mut();
            let temp = slf.user_path.display().to_string();
            if cfg!(unix) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^/|^\.\./|^\./").unwrap();
                }
                if !SL.is_match(&temp) {
                    slf.user_path = PathBuf::from(format!("./{}", slf.user_path.display()));
                }
            }
            if cfg!(windows) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap();
                }
                if !SL.is_match(&temp) {
                    slf.user_path = PathBuf::from(format!(".\\{}", slf.user_path.display()));
                }
            }
        }
        first_slash(self);
        let slf = self.context.get_sync_pack_mut();
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

    fn return_file(path: &str) -> Result<File, Box<dyn Error>> {
        match File::open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(Box::new(err)),
        }
    }

    fn directory_contents(path: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        if let Ok(mut paths) = std::fs::read_dir(path) {
            loop {
                if let Some(item) = paths.next() {
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

    fn dir_create(self: &mut Self, err: std::io::ErrorKind) -> Result<(), Box<dyn Error>> {
        match err {
            std::io::ErrorKind::NotFound => {
                let fullpath = QFilePath::get_path_buf(self)?;
                let filename = fullpath.file_name().unwrap().to_str().unwrap();
                let path_without_file = fullpath.to_str().unwrap().rsplit_once(filename).unwrap().0;
                {
                    self.context.get_sync_pack_mut().user_path = PathBuf::from(path_without_file);
                    self.context.get_sync_pack_mut().update_path = true;
                    self.context.get_sync_pack_mut().file_name = PathBuf::from(filename);
                    self.context.get_sync_pack_mut().flag = Flag::New;
                }
                std::fs::DirBuilder::new()
                    .recursive(true)
                    .create(path_without_file)?;
                Ok(())
            }
            _ => Err(Box::new(QPackError::SyncIOError(SyncIO::IO(err.into())))),
        }
    }
}

pub fn directory_create(slf: &mut QFilePath) -> Result<(), Box<dyn Error>> {
    Ok(fs::DirBuilder::new()
        .recursive(true)
        .create(slf.get_path_buf()?)?)
}

pub fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
    if path_file.as_ref().to_string().is_empty() {
        return Err(Box::new(QPackError::PathIsEmpty));
    }
    let path_file = PathBuf::from(path_file.as_ref());
    if cfg!(unix) {
        if path_file.to_str().unwrap().contains("\\") {
            return Err(Box::new(QPackError::UnixPathIsIncorrect));
        }
    } else if cfg!(windows) {
        if path_file.to_str().unwrap().contains("/") {
            return Err(Box::new(QPackError::UnixPathIsIncorrect));
        }
    } else {
        return Err(Box::new(QPackError::SystemNotDefined));
    }
    Ok(QFilePath {
        context: super::CodeStatus::SyncCode(super::SyncPack {
            request_items: Default::default(),
            user_path: path_file,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: super::Flag::Auto,
            update_path: false,
        }),
    })
}
