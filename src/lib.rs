//! # Qfile
//!
//!  Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.
//! # Example
//! ```rust
//!    // The real file path: `./FOLDER/folder/File.txt`
//!    let mut file = QFilePath::add_path("./folder/folder/file.txt");
//!    // The real path is searched after the first method call.
//!    // It's stored in the structure
//!    file.auto_write("text_1").unwrap();
//!
//!    // we get the saved path right away
//!    file.auto_write("text_2").unwrap();
//!    assert_eq!(file.read().unwrap(), "text_1text_2");
//! ```
//!
//! # Methods
//! - [`add_path`](<struct.QFilePath.html#method.add_path>) - Constructor for storing file data
//! - [`get_file`](<struct.QFilePath.html#method.get_file>) - Get the file directly
//! - [`read`](<struct.QFilePath.html#method.read>) - Get data from a file (`String`)
//! - [`auto_write`](<struct.QFilePath.html#method.auto_write>) -  Auto detect, create or open a file and write data to it (`&str`)
//! - [`write_only_new`](<struct.QFilePath.html#method.write_only_new>) - Overwrite data in the file (`&str`)
//! - [`cache_path`](struct.QFilePath.html#method.cache_path) - Get the true path (`&str`)
mod core;
mod dpds_path;
pub use crate::core::Permissions;
pub use crate::core::QFilePath;
