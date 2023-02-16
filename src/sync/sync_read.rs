use super::super::{Error, QFilePath};
use crate::get_path::get_path_string;
use std::io::Read;
pub fn read(slf: &mut QFilePath) -> Result<String, Box<dyn Error>> {
    let mut text = String::new();
    QFilePath::return_file(&get_path_string(slf)?)?.read_to_string(&mut text)?;
    Ok(text)
}
