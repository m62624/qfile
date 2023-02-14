//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
mod r#async;
mod qerror;
mod sync;
mod systems;
use lazy_static;
pub use qerror::QPackError;
use regex::Regex;
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

#[derive(Debug, Clone)]
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

impl QFilePath {}

impl Drop for QFilePath {
    fn drop(&mut self) {}
}
