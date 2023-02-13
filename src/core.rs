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
}
pub enum Directory {
    ThisPlace(String),
    Everywhere,
}
impl CodeStatus {
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
