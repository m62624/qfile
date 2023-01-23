use self::custom_errors::{QOptionCode, QPathError};
use async_std::sync::Mutex as ACMutex;
mod custom_errors;
mod default;
mod drop;
mod read;
mod write;
//=========================
pub mod r#async;
pub mod r#sync;
//=========================
#[derive(Debug)]
pub enum OptionCodeRequestItems {
    SyncRequestItems(Vec<String>),
    AsyncRequestItems(ACMutex<Vec<String>>),
    UnknownStatusRequestItems,
}
#[derive(Debug)]
pub enum OptionCodeFile {
    SyncFile(std::fs::File),
    AsyncFile(ACMutex<async_std::fs::File>),
    UnknownStatusFile,
}
#[derive(Debug)]
pub enum OptionCodePathBuf {
    SyncPathBuf(std::path::PathBuf),
    AsyncPathBuf(ACMutex<async_std::path::PathBuf>),
    UnknownStatusPathBuf,
}
#[derive(Debug)]
pub enum OptionCodeFlag {
    SyncFlag(Flag),
    AsyncFlag(ACMutex<Flag>),
    UnknownStatusFlag,
}
#[derive(Debug)]
pub enum OptionCodeUpdatePath {
    SyncUpdatePath(bool),
    AsyncUpdatePath(ACMutex<bool>),
    UnknownStatusUpdatePath,
}

#[derive(Debug, Clone, Copy)]
pub enum Flag {
    New,
    Auto,
    Old,
}

#[derive(Debug)]
pub enum QPatternPath {
    NewPattern,
    DefaultPattern,
}
#[derive(Debug)]
pub struct QFilePath {
    request_items: OptionCodeRequestItems,
    only_file: OptionCodeFile,
    user_path: OptionCodePathBuf,
    file_name: OptionCodePathBuf,
    correct_path: OptionCodePathBuf,
    flag: OptionCodeFlag,
    update_path: OptionCodeUpdatePath,
    pattern: QPatternPath,
}
