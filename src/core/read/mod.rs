use std::fs;

use crate::core::get_file;
use crate::core::QFilePack;
use crate::dpds_path::{io, Path, Read};

impl<'a> QFilePack<'a> {
    pub fn read(&mut self) -> Result<String, io::Error> {
        let mut text = String::new();
        match get_file(self.cache_path()) {
            Ok(mut access) => match access.read_to_string(&mut text) {
                Ok(_) => return Ok(text),
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => return Err(err),
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
#[cfg(target_family = "unix")]
#[test]
#[should_panic]
fn test_read_2() {
    let mut file = QFilePack::add_path("");
    let file1 = file.read().unwrap();
    let file2 = file.read().unwrap();
}
#[cfg(target_family = "unix")]
#[test]
fn test_read_3() {
    let mut file = QFilePack::add_path("./polygon/Read/test-3.txt");
    let data = file.read().unwrap();
    let data2 = file.read().unwrap();
    assert_eq!(data, "ok");
    assert_eq!(data2, "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_read_4() {
    let mut file = QFilePack::add_path("root.txt");
    let data = file.read().unwrap();
    dbg!(data);
    assert_eq!(true, true);
}
