pub mod read;
pub mod write;
use crate::dpds_path::io::{self, ErrorKind};
use crate::dpds_path::File;
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err),
            ErrorKind::PermissionDenied => Err(err),
            _ => panic!("::other error::"),
        },
    }
}
