use super::{CodeStatus, Flag, PathBuf, QFilePath, QPackError};
use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;
use std::fs;
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
    fn return_file(path: &str) -> Result<fs::File, Box<dyn Error>> {
        match fs::File::open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(Box::new(err)),
        }
    }
    
}
