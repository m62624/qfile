//! # Qfile
//!
//! Crate for accessing a file by path, case insensitive
//! # Example
//! ```rust
//! use qfile::*;
//!# fn main() {
//! let mut file = QFilePack::add_path("./folder/folder/file.txt");
//! {
//!  // The real path is searched after the first method call.
//!  // It's stored in the structure
//!     file.write("text_1").unwrap();
//! }
//! // we get the saved path right away
//! file.write("text_2").unwrap();
//!
//! println!("{}",file.read().unwrap());
//!
//! //output: text_1text2
//!# }
//! ```
//! 
//! # Methods
//! - [add_path](<struct.QFilePack.html#method.add_path>) - Constructor for storing file data \
//! - [file](<struct.QFilePack.html#method.file>) - Get the file directly\
//! - [read](<struct.QFilePack.html#method.read>) - Getting data from a file (`String`) \
//! - [write](<struct.QFilePack.html#method.write>) - Writing data to a file (`&str`)
mod core;
mod dpds_path;
pub use crate::core::QFilePack;
