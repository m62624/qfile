pub use self::custom_errors::QPackError;
use async_std::fs as AsyncFS;
use async_std::path as AsyncPath;
use async_std::sync::Arc as AsyncArc;
use async_std::sync::Mutex as AsyncMutex;
mod custom_errors;
use async_trait as async_trait_crate;
//=========================
pub mod r#async;
pub mod r#sync;
//=========================
#[derive(Debug, Clone, Copy)]

pub enum Flag {
    Old,
    Auto,
    New,
}
#[derive(Debug)]
pub enum CodeStatus {
    SyncCode(SyncPack),
    AsyncCode(AsyncPack),
}
impl CodeStatus {
    pub fn get_pack_mut(&mut self) -> &mut AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("AsyncPack - `get_pack_mut`")
        }
    }
    pub fn get_pack(&self) -> &AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("AsyncPack - `get_pack`")
        }
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPack {
    request_items: Vec<String>,
    only_file: Option<async_std::fs::File>,
    user_path: async_std::path::PathBuf,
    file_name: async_std::path::PathBuf,
    correct_path: async_std::path::PathBuf,
    flag: Flag,
    update_path: bool,
}
#[derive(Debug)]
pub struct SyncPack {
    request_items: Vec<String>,
    only_file: Option<std::fs::File>,
    user_path: std::path::PathBuf,
    file_name: std::path::PathBuf,
    correct_path: std::path::PathBuf,
    flag: Flag,
    update_path: bool,
}

#[derive(Debug)]
pub struct QFilePath {
    Context: CodeStatus,
}

impl Drop for QFilePath {
    fn drop(&mut self) {}
}
