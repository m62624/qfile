//! # Qfile
//!
//! The crate for working with files without taking into account the case of the path.
//! Automatic detection, create a path with a new file or open an existing file
//! # Usage
//! Add this to your Cargo.toml:
//!```toml
//![dependencies]
//!qfile="0.1.0"
//!```
//! To get started using Rand, see [The Book](https://rust-random.github.io/book).
//! # License
//! [MIT](https://choosealicense.com/licenses/mit/)

mod core;
mod dpds_path;
pub use crate::core::read::file_read;
pub use crate::core::write::{file_write, Flag};
