mod mem;
#[cfg(test)]
mod unit_tests;

use import_libs::*;

mod import_libs {
    pub use home::home_dir;
    pub use std::fs;
    pub use std::fs::File;
    pub use std::io::{Result, Write};
    pub use std::path::PathBuf;
    pub use std::time::Instant;
    pub use sys_info::{disk_info, mem_info};
    pub use unicase::UniCase;
    pub use uuid::Uuid;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodeStatus {
    SyncStatus,
    AsyncStatus,
}

#[derive(Debug, Clone)]
pub struct QFile {
    path: UniCase<PathBuf>,
    code_status: CodeStatus,
}

pub struct IterQfile {
    position: u128,
}
