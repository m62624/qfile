use crate::core::{return_file, Flag, QFilePath};
use crate::dpds_path::{
    io::{self, Write},
    DirBuilder, ErrorKind, File, OpenOptions, Path, PathBuf,
};

impl<'a> QFilePath<'a> {
    /// Auto detect, create or open a file and write data to it
    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    /// // the real file path: `./FILE.txt`
    /// let mut file = QFilePath::add_path("./file.txt");
    /// file.auto_write("ok").unwrap();
    /// assert_eq(file.read().unwrap(),"ok");
    /// # }
    /// ```
    /// - If the path exists, we work with the file (case insensitive)
    ///
    /// | | |
    /// |---|---|
    /// | **The path we specified**: | `folder1/FolDER2/file.TXT` |
    /// | **Real path** : | `./Folder1/Folder2/file.txt` |
    /// | **Result** : | `./Folder1/Folder2/file.txt` |
    /// - If the file/path is not found, creates a new path with the file (*if initial path exists*)
    ///
    /// | | |
    /// |---|---|
    /// | **The path we specified**: | `./folder/folder_new/file.txt` |
    /// | **Real path** : | `./folder` |
    /// | **Result** : | `./folder/folder_new/file.txt` |
    /// - but if the initial path is different case of letters and a new file/folder is specified in the path, then a new path is created with the file
    ///
    /// | | |
    /// |---|---|
    /// | **The path we specified**: | `./FOLDER/Folder_new/file.txt` |
    /// |**Real path** : | `./folder` |
    /// | **Result** :               | `./FOLDER/Folder_new/file.txt` - (**new created path**) |
    /// |                            | `./folder` - (**real path**)
    ///
    pub fn auto_write(&mut self, text: &str) -> Result<(), io::Error> {
        if self.update_path {
            match self.os {
                "linux" | "macos" => {
                    if self.correct_path.to_str().unwrap().is_empty() {
                        self.correct_path = PathBuf::from(format!(
                            "{}{}",
                            self.user_path.to_str().unwrap(),
                            self.file_name.to_str().unwrap()
                        ))
                    } else {
                        self.correct_path = PathBuf::from(format!(
                            "{}/{}",
                            self.correct_path.to_str().unwrap(),
                            self.file_name.to_str().unwrap()
                        ))
                    }
                }
                "windows" => {
                    if self.correct_path.to_str().unwrap().is_empty() {
                        self.correct_path = PathBuf::from(format!(
                            "{}{}",
                            self.user_path.to_str().unwrap(),
                            self.file_name.to_str().unwrap()
                        ))
                    } else {
                        self.correct_path = PathBuf::from(format!(
                            "{}\\{}",
                            self.correct_path.to_str().unwrap(),
                            self.file_name.to_str().unwrap()
                        ))
                    }
                }
                _ => panic!(),
            }
        }
        match self.flag {
            Flag::Auto => match return_file(self.get_path_buf().to_str().unwrap()) {
                Ok(_) => {
                    self.flag = Flag::Old;
                    return self.auto_write(text);
                }
                Err(err) => match err.kind() {
                    _ => {
                        self.dir_create(err.kind()).unwrap();
                        return self.auto_write(text);
                    }
                },
            },

            Flag::New => match File::create(self.get_path_buf()) {
                Ok(_) => {
                    self.update_path = false;
                    self.flag = Flag::Auto;
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(self.get_path_buf())
                        .unwrap()
                        .write_all(text.as_bytes())
                }
                Err(err) => return Err(err),
            },
            Flag::Old => {
                self.flag = Flag::Auto;
                OpenOptions::new()
                    .append(true)
                    .open(self.get_path_buf())
                    .unwrap()
                    .write_all(text.as_bytes())
            }
        }
    }
    fn dir_create(&mut self, err: ErrorKind) -> Result<(), std::io::Error> {
        match err {
            ErrorKind::NotFound => {
                // // self.get_path_buf().to_string();
                let fullpath = self.user_path.clone();
                let filename = fullpath.file_name().unwrap().to_str().unwrap();
                let path_without_file = fullpath.to_str().unwrap().rsplit_once(filename).unwrap().0;
                {
                    self.user_path = PathBuf::from(path_without_file);
                    // self.correct_path();
                    self.update_path = true;
                    self.file_name = PathBuf::from(filename);
                    self.flag = Flag::New;
                }
                DirBuilder::new()
                    .recursive(true)
                    .create(self.get_path_buf())
                    .unwrap();
                Ok(())
            }
            ErrorKind::PermissionDenied => {
                panic!("Permission denied");
            }
            _ => panic!("other errors"),
        }
    }
    /// The same as [`auto_write()`](<struct.QFilePath.html#method.auto_write>), only the method for overwriting the data in the file
    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    /// // the real file path: `file_Temp.txt`
    /// // file content: `1 2 3`
    /// let mut file = QFilePath::add_path("File_temp.txt");
    /// file.write_only_new("4 5 6").unwrap();
    /// assert_eq(file.read().unwrap(),"4 5 6");
    /// # }
    /// ```
    pub fn write_only_new(&mut self, text: &str) -> Result<(), io::Error> {
        self.flag = Flag::New;
        if let Err(err) = self.auto_write(text) {
            self.dir_create(err.kind()).unwrap();
            self.auto_write(text).unwrap();
        }
        Ok(())
    }
}
