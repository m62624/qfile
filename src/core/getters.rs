use super::{AsyncPack, CodeStatus, QFilePath, SyncPack};
impl AsyncPack {
    pub async fn async_get_mut(&mut self) -> &mut Self {
        self
    }
}
impl SyncPack {
    pub fn get_syn_pack(&self) -> &Self {
        self
    }
    pub fn get_syn_pack_mut(&mut self) -> &mut Self {
        self
    }
}
impl CodeStatus {
    pub async fn get_async_pack_mut(&mut self) -> &mut AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("AsyncPack - `get_async_pack_mut`")
        }
    }
    pub async fn get_async_pack(&self) -> &AsyncPack {
        if let Self::AsyncCode(value) = self {
            value
        } else {
            panic!("AsyncPack - `get_async_pack`")
        }
    }
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
impl QFilePath {
    pub fn get_context_mut(&mut self) -> &mut CodeStatus {
        &mut self.Context
    }
}
