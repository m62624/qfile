use crate::core::get_file;
use crate::core::QFilePack;
use crate::dpds_path::{io, Read};

impl<'a> QFilePack<'a> {
    pub fn read(&self) -> Result<String, io::Error> {
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
//=====================================(tests)=====================================
#[cfg(target_family = "unix")]
#[test]
fn test_read_1() {
    let file = QFilePack::add_path("./Polygon/Read/test-1.txt");
    let file = file.read().unwrap();
    assert_eq!(file, "ok");
}
