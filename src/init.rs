// use self::constructor::add_path;

use super::{CodeStatus, Flag, PathBuf, QFilePath, QPackError};
use async_fs;
use futures_lite::stream::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;
use std::{error::Error, fs, path::Path};
mod constructor {
    use super::*;
    fn core<T: AsRef<str>>(path_file: T, status: CodeStatus) -> Result<QFilePath, Box<dyn Error>> {
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
                return Err(Box::new(QPackError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPackError::SystemNotDefined));
        }
        Ok(QFilePath {
            request_items: Default::default(),
            user_path: path_file,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Flag::Auto,
            update_path: false,
            status,
        })
    }
    pub fn add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        core(path_file, CodeStatus::SyncStatus)
    }
    pub async fn async_add_path<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        core(path_file, CodeStatus::AsyncStatus)
    }
}

mod path_separation {
    use super::*;
    fn core(slf: &mut QFilePath) {
        fn first_slash(sl: &mut QFilePath) {
            let temp = sl.user_path.display().to_string();
            lazy_static! {
                static ref SL: Regex = {
                    #[cfg(unix)]
                    {
                        Regex::new(r"^/|^\.\./|^\./").unwrap()
                    }
                    #[cfg(windows)]
                    {
                        Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap()
                    }
                };
            }
            if !SL.is_match(&temp) {
                sl.user_path = PathBuf::from(format!(
                    "{}{}",
                    {
                        #[cfg(unix)]
                        {
                            "./"
                        }
                        #[cfg(windows)]
                        {
                            ".\\"
                        }
                    },
                    sl.user_path.display()
                ));
            }
        }
        first_slash(slf);
        slf.request_items = slf
            .user_path
            .ancestors()
            .map(|element| element.display().to_string())
            .collect();
        if slf.request_items.last().unwrap().eq("") {
            slf.request_items.pop();

            if let Some(value) = slf.request_items.last_mut() {
                #[cfg(unix)]
                {
                    if value.eq(&mut ".") {
                        *value = String::from("./")
                    }
                    if value.eq(&mut "..") {
                        *value = String::from("../")
                    }
                }
                #[cfg(windows)]
                {
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
    pub fn way_step_by_step(slf: &mut QFilePath) {
        core(slf)
    }
    pub async fn async_way_step_by_step(slf: &mut QFilePath) {
        core(slf)
    }
}

mod work_with_elements {
    use super::*;
    pub fn directory_contents(path: &str) -> Vec<String> {
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
    pub async fn async_directory_contents(path: &str) -> Vec<String> {
        let mut files: Vec<String> = Vec::new();
        if let Ok(mut paths) = async_fs::read_dir(path).await {
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
    pub fn return_file(path: &str) -> Result<fs::File, Box<dyn Error>> {
        match fs::File::open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(Box::new(err)),
        }
    }
    pub async fn async_return_file(path: &str) -> Result<async_fs::File, Box<dyn Error>> {
        match async_fs::File::open(path).await {
            Ok(file) => Ok(file),
            Err(err) => Err(Box::new(err)),
        }
    }
}

mod correct_path {
    use super::path_separation::*;
    use super::work_with_elements::*;
    use super::*;
    fn core(
        slf: &mut QFilePath,
        directory_cnt: Vec<String>,
        user_i: usize,
        counter: &mut usize,
        len: usize,
    ) -> Result<(), Box<dyn Error>> {
        // let mut counter = 0;
        // for user_i in 0..slf.request_items.len() {
        let mut possible_directories = directory_cnt;
        //directory_contents(slf.request_items[user_i].as_str());
        for pos_j in 0..possible_directories.len() {
            if slf
                .request_items
                .get(user_i + 1)
                .unwrap_or(&slf.request_items.get(user_i).unwrap().to_lowercase())
                .to_lowercase()
                == possible_directories[pos_j].to_lowercase()
            {
                slf.request_items[user_i + 1] = possible_directories.remove(pos_j);
                *counter += 1;
                break;
            }
        }
        // }
        if user_i < len - 1 {
            if Path::new(slf.request_items.last().unwrap()).exists() {
                slf.correct_path = PathBuf::from(slf.request_items.last().unwrap());
            } else if cfg!(unix) {
                if Path::new(&slf.request_items[*counter]).exists() && *counter != (0 as usize) {
                    slf.correct_path = PathBuf::from(format!(
                        "{}{}",
                        slf.request_items[*counter],
                        slf.request_items
                            .last()
                            .unwrap()
                            .split_at(slf.request_items[*counter].len())
                            .1
                    ));
                }
            }
        }
        Ok(())
    }
    // fn _core(slf: &mut QFilePath, status: CodeStatus) -> Result<(), Box<dyn Error>> {
    //     let mut counter = 0;
    //     if slf.request_items.is_empty() {
    //         way_step_by_step(slf);
    //     }

    // }
    pub fn correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error>> {
        let mut counter = 0;
        if slf.request_items.is_empty() {
            way_step_by_step(slf);
        };
        let len = slf.request_items.len();
        for user_i in 0..len {
            core(
                slf,
                directory_contents(slf.request_items[user_i].as_str()),
                user_i,
                &mut counter,
                len,
            )?;
        }
        Ok(())
        // core(slf, CodeStatus::SyncStatus)
    }
    pub async fn async_correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error>> {
        let mut counter = 0;
        if slf.request_items.is_empty() {
            async_way_step_by_step(slf).await;
        };
        let len = slf.request_items.len();
        for user_i in 0..len {
            core(
                slf,
                async_directory_contents(slf.request_items[user_i].as_str()).await,
                user_i,
                &mut counter,
                len,
            )?;
        }
        Ok(())
    }
}
#[test]
fn check_correct_path() {
    let mut qfile = self::constructor::add_path("SRC").unwrap();
    self::correct_path::correct_path(&mut qfile).unwrap();
    dbg!(qfile);
}
