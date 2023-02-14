//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
mod QError;
mod r#async;
mod core;
mod sync;
pub use QError::QPackError;
