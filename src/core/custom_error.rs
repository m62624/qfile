use std::{env, io};

use crate::dpds_path::{Display, Error};
/// Custom error to avoid merging paths of different systems
/// # Example
///```
/// use std::error::Error;
/// use std::io::Read;
/// // Linux
/// use qfile::*;
/// fn example() -> Result<String, Box<dyn Error>> {
///     let (mut file, mut content) = (
///         // returns the type OsPathError
///         QFilePath::add_path("src\\file.rs")?.get_file(Permissions::RW)?,
///         String::new(),
///     );
///     file.read_to_string(&mut content)?;
///     return Ok(content);
/// }
/// fn main() {
///     if let Err(err) = example() {
///         println!("{err}");
///     }
/// }
///```
///
/// ---
/// Output:
///  > You are using the windows path format for Unix. Use `unix` format for the path\
///  > \> ./folder1/folder2/file.txt\
///  > \> ../folder2/file.txt\
///  > \> ./file.txt
///
#[derive(Debug)]

pub enum OsPathError {
    UnixPathIncorrect,
    WindowsPathIncorrect,
    SystemNotDefined,
}
impl Error for OsPathError {}
impl Display for OsPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OsPathError::UnixPathIncorrect => f.write_str("You are using the windows path format for Unix. Use `unix` format for the path:\n> ./folder1/folder2/file.txt\n> ../folder2/file.txt\n> ./file.txt"),
            OsPathError::WindowsPathIncorrect => f.write_str("You are using the unix path format for Windows. Use `windows` format for the path:\n> .\\folder1\\folder2\\file.txt\n> ..\\folder2\\file.txt\n> .\\file.txt"),
            OsPathError::SystemNotDefined =>f.write_str(" SystemNotDefined"),
        }
    }
}
