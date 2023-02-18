//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
mod find;
mod init;
mod paths;
mod qerror;
mod read;
mod write;
pub use qerror::QPackError;
use std::path::PathBuf;
mod prelude_async;
mod prelude_sync;
#[derive(Debug, Clone)]
enum Flag {
    Old,
    Auto,
    New,
}

#[derive(Debug, Clone)]
pub enum Directory<T: AsRef<str> + Send + Sync + 'static> {
    ThisPlace(Vec<T>),
    Everywhere,
}

#[derive(Debug, Clone, PartialEq)]
enum CodeStatus {
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
    status: CodeStatus,
}
impl QFilePath {
    fn check_status_code(&self, status: CodeStatus) -> Result<(), QPackError> {
        let check = |err_st: QPackError| -> Result<(), QPackError> {
            if self.status == status {
                return Ok(());
            }
            return Err(err_st);
        };
        match self.status {
            CodeStatus::SyncStatus => check(QPackError::AsyncCallFromSync),
            CodeStatus::AsyncStatus => check(QPackError::SyncCallFromAsync),
        }
    }
}
impl Drop for QFilePath {
    fn drop(&mut self) {}
}
