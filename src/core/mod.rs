pub mod read;
pub mod write;
use crate::dpds_path::io::{self, ErrorKind};
use crate::dpds_path::File;

#[test]
fn check_get_file() {
    match get_file("sdsd") {
        Ok(_) => println!("ok"),
        Err(e) => println!("No problem {}", e),
    }
}
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err.kind().into()),
            ErrorKind::PermissionDenied => Err(err.kind().into()),
            _ => panic!("::other error::"),
        },
    }
}
