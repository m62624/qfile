//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
mod r#async;
mod get_path;
mod qerror;
mod sync;
use lazy_static::lazy_static;
pub use qerror::QPackError;
use regex::Regex;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Flag {
    Old,
    Auto,
    New,
}

#[derive(Debug, Clone)]
pub enum Directory {
    ThisPlace(String),
    Everywhere,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodeStatus {
    SyncStatus,
    AsyncStatus,
}

#[derive(Debug, Clone)]
pub struct QFilePath {
    request_items: Vec<String>,
    user_path: PathBuf,
    file_name: PathBuf,
    correct_path: PathBuf,
    flag: Flag,
    update_path: bool,
    status_mod: CodeStatus,
}

impl QFilePath {
    fn way_step_by_step(&mut self) {
        fn first_slash(sl: &mut QFilePath) {
            let temp = sl.user_path.display().to_string();
            if cfg!(unix) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^/|^\.\./|^\./").unwrap();
                }
                if !SL.is_match(&temp) {
                    sl.user_path = PathBuf::from(format!("./{}", sl.user_path.display()));
                }
            }
            if cfg!(windows) {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap();
                }
                if !SL.is_match(&temp) {
                    sl.user_path = PathBuf::from(format!(".\\{}", sl.user_path.display()));
                }
            }
        }
        first_slash(self);
        self.request_items = self
            .user_path
            .ancestors()
            .map(|element| element.display().to_string())
            .collect();
        if self.request_items.last().unwrap().eq("") {
            self.request_items.pop();

            if let Some(value) = self.request_items.last_mut() {
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
        self.request_items.reverse();
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
}
impl Drop for QFilePath {
    fn drop(&mut self) {}
}
