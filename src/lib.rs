//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
mod core;
pub use crate::core::{r#async::async_trait, sync::sync_trait, QFilePath, QPackError};
