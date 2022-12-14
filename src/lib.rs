//! # Qfile
//!
//! Crate for accessing a file by path, case insensitive
//!
//! # Example
//! ```
//! use qfile::*;
//!# fn main() {
//! //---
//!    let mut file = QFilePack::add_path("./Folder1/Folder2/file.txt");
//!    file.write("text_1").unwrap();
//!    let data = file.read().unwrap();
//!    println!("{}",data);
//! //---
//!# }
//!
//! ```
//! # Methods
//! - [add_path](<struct.QFilePack.html#method.add_path>) - Constructor for storing file data \
//! - [file](<struct.QFilePack.html#method.file>) - Get the file directly\
//! - [read](<struct.QFilePack.html#method.read>) - Getting data from a file (`String`) \
//! - [write](<struct.QFilePack.html#method.write>) - Writing data to a file (`&str`)
mod core;
mod dpds_path;
pub use crate::core::QFilePack;
