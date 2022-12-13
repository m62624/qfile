use crate::core::Pack;
use crate::dpds_path::{io, ErrorKind, File, Read};
// Получить файл
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err),
            ErrorKind::PermissionDenied => Err(err),
            _ => panic!(":: other errors ::"),
        },
    }
}
impl<'a> Pack<'a> {
    pub fn file_read(self) -> Result<String, io::Error> {
        let mut text = String::new();
        match get_file(self.user_path) {
            Ok(mut access) => match access.read_to_string(&mut text) {
                Ok(_) => return Ok(text),
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}
