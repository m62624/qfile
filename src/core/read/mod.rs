use crate::core::get_file;
use crate::core::QFilePack;
use crate::dpds_path::{io, Read};

impl<'a> QFilePack<'a> {
    /// The method returns the contents of the file
    /// # Example
    /// ```
    /// use qfile::QFilePack;
    /// # fn main() {
    /// //---
    /// // the real file path:
    /// // ./FOLDER/File.txt
    /// let mut file = QFilePack::add_path("./folder/file.txt");
    /// let data = file.read().unwrap();
    /// //on re-reading, use the correct path from the cache
    /// // {
    /// //  let data2 = file.read().unwrap();
    /// // }
    /// assert_eq!(data, "ok");
    /// //---
    /// # }
    /// // file content:
    /// // ok
    ///```
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
    use std::path::Path;
    //==============================================================
    let mut file = QFilePack::add_path("./Polygon/READ/Test-1.txt");
    if !Path::new("./Polygon/Read/test-1.txt").exists() {
        file.write("ok").unwrap();
    }
    //==============================================================

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
    {
        let _file1 = file.read().unwrap();
    }
    let _file2 = file.read().unwrap();
}
#[cfg(target_family = "unix")]
#[test]
fn test_read_3() {
    use std::path::Path;
    //==============================================================
    let mut file = QFilePack::add_path("./polygon/Read/test-3.txt");
    if !Path::new("./Polygon/Read/test-3.txt").exists() {
        file.write("ok").unwrap();
    }
    //==============================================================

    let data = file.read().unwrap();
    let data2 = file.read().unwrap();

    assert_eq!(data, "ok");
    assert_eq!(data2, "ok");
}
