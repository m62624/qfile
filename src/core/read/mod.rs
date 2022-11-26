use crate::dpds_path::io::{self, Read};

use crate::core::get_file;

pub fn file_read(path: &str) -> Result<String, io::Error> {
    let mut text = String::new();
    if let Err(err) = get_file(path).unwrap().read_to_string(&mut text) {
        return Err(err.into());
    }
    Ok(text)
}
