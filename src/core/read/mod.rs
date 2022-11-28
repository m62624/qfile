use crate::core::write::files::correct_path;
use crate::dpds_path::io;
pub mod only_for_crate {
    use super::io::{self, Read};
    use crate::core::get_file;
    pub fn file_read(path: &str) -> Result<String, io::Error> {
        let mut text = String::new();
        match get_file(&path) {
            Ok(mut access) => match access.read_to_string(&mut text) {
                Ok(_) => return Ok(text),
                Err(err) => {
                    return Err(err.kind().into());
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}
// Function for reading a file
/// searches for a file in a path, *case insensitive*
/// # Examples
/// ```
/// //Creating a new file to read
///file_write("./Files/new.txt","text text text",Flag::Auto).unwrap();
///assert_eq!(file_read(""./files/new.txt""),"text text text");
///
/// ```
pub fn file_read(path: &str) -> Result<String, io::Error> {
    let temp = &correct_path(path).unwrap();
    match only_for_crate::file_read(temp) {
        Ok(result) => Ok(result),
        Err(err) => return Err(err.kind().into()),
    }
}
