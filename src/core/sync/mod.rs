pub mod sync_trait;
pub use self::sync_trait::SyncQPackTrait;
use super::{CodeStatus, SyncPack};
impl SyncPack {
    pub fn get_sync(&self) -> &Self {
        self
    }
    pub fn get_sync_mut(&mut self) -> &mut Self {
        self
    }
}
impl CodeStatus {
    pub fn get_sync_pack_mut(&mut self) -> &mut SyncPack {
        if let Self::SyncCode(value) = self {
            value
        } else {
            panic!("SyncPack - `get_sync_pack_mut`")
        }
    }
    pub fn get_sync_pack(&self) -> &SyncPack {
        if let Self::SyncCode(value) = self {
            value
        } else {
            panic!("SyncPack - `get_sync_pack`")
        }
    }
}
