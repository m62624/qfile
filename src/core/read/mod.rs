use crate::dpds_path::io::{self, Read};

use crate::core::get_file;
#[test]
fn check_file_read() {
    match file_read("sadsdsda") {
        Ok(_) => println!("ok"),
        Err(_) => println!("No problem"),
    }
}
pub fn file_read(path: &str) -> Result<String, io::Error> {
    let mut text = String::new();
    // if let Err(err) = get_file(path).unwrap().read_to_string(&mut text) {
    //     return Err(err);
    // }
    // Ok(text)
    match get_file(path).unwrap().read_to_string(&mut text) {
        Ok(_) => Ok(text),
        Err(err) => Err(err),
    }
}
