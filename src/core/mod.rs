pub use self::custom_errors::QPackError;
use async_std::{
    fs as AsyncFS, path as AsyncPath,
    sync::{Arc, Mutex as AsyncMutex},
};
mod custom_errors;
use async_trait as async_trait_crate;
//=========================
pub mod r#async;
pub mod r#sync;
//=========================
#[derive(Debug, Clone)]
pub enum Flag {
    Old,
    Auto,
    New,
}
#[derive(Debug, Clone)]
pub enum CodeStatus {
    SyncCode(SyncPack),
    AsyncCode(AsyncPack),
}
impl CodeStatus {
    pub fn get_async_pack_mut(&mut self) -> &mut AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("{}", QPackError::AsyncCallFromSync);
        }
    }
    pub fn get_async_pack(&self) -> &AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("{}", QPackError::AsyncCallFromSync);
        }
    }
    pub fn get_sync_pack_mut(&mut self) -> &mut SyncPack {
        if let Self::SyncCode(value) = self {
            value
        } else {
            panic!("{}", QPackError::SyncCallFromAsync)
        }
    }
    pub fn get_sync_pack(&self) -> &SyncPack {
        if let Self::SyncCode(value) = self {
            value
        } else {
            panic!("{}", QPackError::SyncCallFromAsync)
        }
    }
}
#[derive(Debug, Clone)]
pub struct AsyncPack {
    request_items: Vec<String>,
    user_path: async_std::path::PathBuf,
    file_name: async_std::path::PathBuf,
    correct_path: async_std::path::PathBuf,
    flag: Flag,
    update_path: bool,
}
#[derive(Debug, Clone)]
pub struct SyncPack {
    request_items: Vec<String>,
    user_path: std::path::PathBuf,
    file_name: std::path::PathBuf,
    correct_path: std::path::PathBuf,
    flag: Flag,
    update_path: bool,
}

#[derive(Debug, Clone)]
pub struct QFilePath {
    context: CodeStatus,
}

impl Drop for QFilePath {
    fn drop(&mut self) {}
}
