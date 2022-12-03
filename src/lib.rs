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
//! # Example
//! ```
//! use qfile::{file_read, file_write, Flag};
//!fn main() {
//!    file_write(
//!        "./Folder1/NewFolder1/file_new.txt",
//!        "TEXT TEXT TEXT",
//!        Flag::Auto,
//!    )
//!    .unwrap();
//!    println!("{}",file_read("./Folder1/NewFolder1/file_new.txt").unwrap());
//!}
//! ```
//! **more information** in the [FLAG](enum.Flag.html)
//! 
//! # License
//! [MIT](https://choosealicense.com/licenses/mit/)

mod core;
mod dpds_path;
pub use crate::core::read::file_read;
pub use crate::core::write::{file_write, Flag};
