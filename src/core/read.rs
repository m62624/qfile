use crate::core::{return_file, QFilePath};
use crate::dpds_path::{io, Read};

impl<'a> QFilePath<'a> {
    /// The method returns the contents of the file
    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    /// // the real file path: `./FOLDER/File.txt`
    /// let mut file = QFilePath::add_path("./folder/file.txt");
    ///  //on re-reading, use the correct path from the cache
    /// let data = file.read().unwrap();
    /// assert_eq!(data, "ok");
    /// # }
    ///
    /// ```
    pub fn read(&mut self) -> Result<String, io::Error> {
        let mut text = String::new();
        match return_file(self.cache_path()) {
            Ok(mut access) => match access.read_to_string(&mut text) {
                Ok(_) => return Ok(text),
                Err(err) => return Err(err),
            },
            Err(err) => return Err(err),
        }
    }
}
