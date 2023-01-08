use crate::core::{get_file, Flag, QFilePack};
use crate::dpds_path::{
    io::{self, Write},
    DirBuilder, ErrorKind, File, OpenOptions,
};

impl<'a> QFilePack<'a> {
    /// Method for writing data to a file
    /// # Example
    /// ```
    /// # use qfile::QFilePack;
    /// # fn main() {
    /// // the real file path: `./FILE.txt`
    /// let mut file = QFilePack::add_path("./file.txt");
    /// file.write("ok").unwrap();
    /// assert_eq(file.read().unwrap(),"ok");
    /// # }
    /// ```
    ///  - If the path exists, we work with the file (case insensitive)
    ///  > **The path we specified**: `./FLDR/FlDr/file.TXT`\
    ///  **real path** : `./fldr/fldr/file.txt`\
    ///  **Result** : `"./fldr/fldr/file.txt"`
    /// - If the file/path is not found, creates a new path with the file (*if initial path exists*)
    /// > **The path we specified**: `./fldr/fldr_new/file.txt`\
    ///  **real path** : `./fldr`\
    ///  **Result** : `./fldr/fldr_new/file.txt`
    /// - but if the initial path is different case of letters and a new file/folder is specified in the path, then a new path is created with the file
    /// > **The path we specified**: `./FLDR/fldr_new/file.TXT`\
    ///  **real path** : `./fldr`\
    ///  **Result** :\
    ///  `./fldr`\
    ///  `./FLDR/fldr_new/file.TXT`
    pub fn write(&mut self, text: &str) -> Result<(), io::Error> {
        if self.update_path {
            match self.os {
                "linux" | "macos" => {
                    if self.correct_path.is_empty() {
                        self.correct_path = format!("{}{}", self.user_path, self.file_name)
                    } else {
                        self.correct_path =
                            format!("{}{}", self.correct_path.clone(), self.file_name)
                    }
                }
                "windows" => {
                    if self.correct_path.is_empty() {
                        self.correct_path = format!("{}{}", self.user_path, self.file_name)
                    } else {
                        self.correct_path =
                            format!("{}{}", self.correct_path.clone(), self.file_name)
                    }
                }
                _ => panic!(),
            }
        }
        match self.flag {
            Flag::Auto => match get_file(self.cache_path()) {
                Ok(_) => {
                    self.flag = Flag::Old;
                    return self.write(text);
                }
                Err(err) => match err.kind() {
                    _ => {
                        self.dir_create(err.kind()).unwrap();
                        return self.write(text);
                    }
                },
            },

            Flag::New => match File::create(self.cache_path()) {
                Ok(_) => {
                    self.update_path = false;
                    self.flag = Flag::Auto;
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(self.cache_path())
                        .unwrap()
                        .write_all(text.as_bytes())
                }
                Err(err) => return Err(err),
            },
            Flag::Old => {
                self.flag = Flag::Auto;
                OpenOptions::new()
                    .append(true)
                    .open(self.cache_path())
                    .unwrap()
                    .write_all(text.as_bytes())
            }
        }
    }
    fn dir_create(&mut self, err: ErrorKind) -> Result<(), std::io::Error> {
        match err {
            ErrorKind::NotFound => {
                self.cache_path().to_string();
                let fullpath = self.user_path;
                let filename = match self.os {
                    "linux" | "macos" => match fullpath.rsplit_once("/") {
                        Some(filename) => filename.1,
                        None => self.user_path,
                    },
                    "windows" => match fullpath.rsplit_once("\\") {
                        Some(filename) => filename.1,
                        None => self.user_path,
                    },
                    _ => panic!(),
                };
                let path_without_file = {
                    let mut temp = fullpath.rsplit_once(filename).unwrap().0;
                    if temp.is_empty() {
                        match self.os {
                            "linux" | "macos" => temp = "./",
                            "windows" => temp = ".\\",
                            _ => panic!(),
                        }
                    }
                    temp.split_at(temp.len() - 1).0;
                    // }
                    temp
                };
                {
                    self.user_path = path_without_file;
                    self.correct_path();
                    self.update_path = true;
                    self.file_name = filename;
                    self.flag = Flag::New;
                }
                DirBuilder::new()
                    .recursive(true)
                    .create(self.cache_path())
                    .unwrap();
                Ok(())
            }
            ErrorKind::PermissionDenied => {
                panic!("Permission denied");
            }
            _ => panic!("other errors"),
        }
    }
    /// The same as `write()`, only the method for overwriting the data in the file
    /// # Example
    /// ```
    /// # use qfile::QFilePack;
    /// # fn main() {
    /// // the real file path: `./FILE.txt`
    /// // file content: `1 2 3`
    /// let mut file = QFilePack::add_path("./file.txt");
    /// file.write_only_new("4 5 6").unwrap();
    /// assert_eq(file.read().unwrap(),"4 5 6");
    /// # }
    /// ```
    pub fn write_only_new(&mut self, text: &str) -> Result<(), io::Error> {
        self.flag = Flag::New;
        if let Err(err) = self.write(text) {
            self.dir_create(err.kind()).unwrap();
            self.write(text).unwrap();
        }
        Ok(())
    }
}
