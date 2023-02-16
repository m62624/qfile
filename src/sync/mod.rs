use super::{Error, QFilePath};
use std::io::Read;
pub fn read(slf: &mut QFilePath) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    QFilePath::return_file(&slf.)
}
/*
pub fn read(slf: &mut QFilePath) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    QFilePath::return_file(&slf.get_path_string()?)?.read_to_string(&mut text)?;
    Ok(text)
} */
