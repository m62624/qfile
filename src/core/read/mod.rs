use std::io::ErrorKind;

use crate::dpds_path::io::{self, Read};

use crate::core::get_file;
#[test]
fn check_file_read() {
    match file_read("sadsdsda") {
        Ok(_) => println!("ok"),
        Err(e) => println!("No problem {}", e),
    }
}
// pub fn file_read(path: &str) -> Result<String, io::Error> {
//     let mut text = String::new();
//     if let Err(err) = get_file(path).unwrap().read_to_string(&mut text) {
//         // return Err(err.kind().into());
//         match err.kind() {
//             ErrorKind::NotFound => return Err(err),
//             _ => panic!("file_read-err"),
//         }
//     }
//     Ok(text)
//     // match get_file(path).unwrap().read_to_string(&mut text) {
//     //     Ok(_) => Ok(text),
//     //     Err(err) => Err(err),
//     // }
// }
pub fn file_read(path: &str) -> Result<String, io::Error> {
    let mut text = String::new();
    match get_file(path) {
        Ok(mut access) => match access.read_to_string(&mut text) {
            Ok(_) => return Ok(text),
            Err(err) => {
                println!("error 2 level: {}", err);
                return Err(err.kind().into());
            }
        },
        Err(err) => {
            println!("error 1 level: {}", err);
            return Err(err.kind().into());
        }
    }
}
