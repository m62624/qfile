use crate::core::get_file;
use crate::core::QFilePack;
use crate::dpds_path::{io, Read};

impl<'a> QFilePack<'a> {
    pub fn read(&mut self) -> Result<String, io::Error> {
        let mut text = String::new();
        let temp_path = if self.correct_path.is_empty() {
            self.correct_path();
            self.correct_path.as_str()
        } else {
            self.correct_path.as_str()
        };
        match get_file(temp_path) {
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
    let mut file = QFilePack::add_path("./Polygon/READ/Test-1.txt");
    let data = file.read().unwrap();
    let data2 = file.read().unwrap();
    assert_eq!(data, "ok");
    assert_eq!(data2, "ok");
}
#[test]
#[should_panic]
fn test_read_2() {
    let mut file = QFilePack::add_path("");
    let file1 = file.read().unwrap();
    let file2 = file.read().unwrap();
}
