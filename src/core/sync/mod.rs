mod sync_read;
pub mod sync_trait;
mod sync_write;
use std::error::Error;
use std::path::PathBuf;

use crate::{QFilePath, QPackError};
fn add_path_for_async<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {}
