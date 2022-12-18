use crate::core::{get_file, QFilePack};
use crate::dpds_path::{io, Read};

impl<'a> QFilePack<'a> {
    /// The method returns the contents of the file
    /// # Example
    /// ```rust
    /// # use qfile::QFilePack;
    /// # fn main() {
    /// // the real file path:
    /// // ./FOLDER/File.txt
    /// let mut file = QFilePack::add_path("./folder/file.txt");
    /// let data = file.read().unwrap();
    /// //on re-reading, use the correct path from the cache
    /// {
    ///     let data2 = file.read().unwrap();
    /// }
    /// assert_eq!(data, "ok");
    /// # }
    ///
    /// // file content:
    /// // ok
    ///
    /// ```
    ///
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
