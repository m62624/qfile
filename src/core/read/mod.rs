use super::write::files::correct_path;
use crate::core::get_file;
use crate::dpds_path::io::{self, Read};
pub fn file_read(path: &str) -> Result<String, io::Error> {
    let mut text = String::new();
    match get_file(&path) {
        Ok(mut access) => match access.read_to_string(&mut text) {
            Ok(_) => return Ok(text),
            Err(err) => {
                // println!("error 2 level: {}", err);
                return Err(err);
            }
        },
        Err(err) => {
            // println!("error 1 level: {}", err);
            return Err(err);
        }
    }
}
#[test]
fn test_read_check() {
    let temp = &correct_path("./Polygon/correctpath2/new.txt").unwrap();
    println!("{}", file_read(temp).unwrap());
}
