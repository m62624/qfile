mod custom_error;
mod read;
mod write;
use crate::dpds_path::{env, fs, io, lazy_static, File, OpenOptions, Path, PathBuf, Regex};
pub use custom_error::OsPathError;
#[derive(Debug)]
pub enum Flag {
    New,
    Auto,
    Old,
}
/// Enumeration for selecting read and write permissions
#[derive(Debug)]
pub enum Permissions {
    /// Read mode
    R,
    /// Write mode
    W,
    /// Read and write mode
    RW,
}
/// The structure for storing the file path
///
/// The structure stores :
/// - true file path (**used as a [cache](<struct.QFilePath.html#method.add_path>) for reuse**)
/// - OS (information about what format to look for the file `/` and `\\`)\
/// (**All methods automatically find the path case insensitive**)
#[derive(Debug)]
pub struct QFilePath<'a> {
    request_items: Vec<String>,
    //================
    user_path: PathBuf,
    file_name: PathBuf,
    correct_path: PathBuf,
    //================
    os: &'a str,
    flag: Flag,
    update_path: bool,
}

//======================================================
impl<'a> QFilePath<'a> {
    /// Constructor for adding a file path.\
    /// After using the [`auto_write()`](<struct.QFilePath.html#method.auto_write>) or [`read()`](<struct.QFilePath.html#method.read>) methods (also the [`get_path_buf() | get_path_str()`](<struct.QFilePath.html#method.get_path_buf>) if the path exists), and if `Ok`,\
    /// we get the correct path, which will be used as a cache when we reuse
    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    /// // the real file path: `./FOLder/Folder/NEW.txt`
    /// let mut file = QFilePath::add_path("./folder/Folder/new.txt").unwrap();
    /// // The real path is searched after the first method call
    /// // (it's stored in the structure).
    /// file.auto_write("Olddata").unwrap();
    /// // we get the saved path right away
    /// file.auto_write("Newdata").unwrap();
    /// assert_eq!(file.read().unwrap(), "OlddataNewdata");
    /// # }
    /// ```
    ///
    /// ## Linux :
    ///
    /// |                            |                                                         |
    /// | -------------------------- | ------------------------------------------------------- |
    /// | **The path we specified**: | `./FOLDER/Folder_new/file.txt`                          |
    /// | **Real path** :            | `./folder`                                              |
    /// | **Result** :               | `./folder/Folder_new/file.txt`                          |
    ///
    /// ## Windows :
    ///
    /// |                            |                                                  |
    /// | -------------------------- | ------------------------------------------------ |
    /// | **The path we specified**: | `.\FOLDER\Folder_new\file.txt`                   |
    /// | **Real path** :            | `.\folder`                                       |
    /// | **Result** :               | `.\folder\Folder_new\file.txt`                   |
    ///
    pub fn add_path<T: ToString>(path_file: T) -> Result<Self, OsPathError> {
        let path_file = PathBuf::from(path_file.to_string());
        match env::consts::OS {
            "windows" => {
                if path_file.to_str().unwrap().contains("/") {
                    return Err(OsPathError::WindowsPathIncorrect);
                }
            }
            "linux" | "macos" => {
                if path_file.to_str().unwrap().contains("\\") {
                    return Err(OsPathError::UnixPathIncorrect);
                }
            }
            _ => return Err(OsPathError::SystemNotDefined),
        }
        Ok(Self {
            os: env::consts::OS,
            user_path: path_file,
            flag: Flag::Auto,
            update_path: false,
            request_items: Default::default(),
            correct_path: Default::default(),
            file_name: Default::default(),
        })
    }
    fn first_slah(&mut self) {
        let temp = self.user_path.display().to_string();
        match self.os {
            "windows" => {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap();
                }
                if !SL.is_match(&temp) {
                    self.user_path = PathBuf::from(format!(".\\{}", self.user_path.display()));
                }
            }
            "linux" | "macos" => {
                lazy_static! {
                    static ref SL: Regex = Regex::new(r"^/|^\.\./|^\./").unwrap();
                }
                if !SL.is_match(&temp) {
                    self.user_path = PathBuf::from(format!("./{}", self.user_path.display()));
                }
            }
            _ => panic!("Incorrect path"),
        }
    }

    fn way_step_by_step(&mut self) {
        self.first_slah();
        self.request_items = self
            .user_path
            .ancestors()
            .map(|element| element.display().to_string())
            .collect();
        if self.request_items.last().unwrap().eq("") {
            self.request_items.pop();

            if let Some(value) = self.request_items.last_mut() {
                match self.os {
                    "linux" | "macos" => {
                        if value.eq(&mut ".") {
                            *value = String::from("./")
                        }
                        if value.eq(&mut "..") {
                            *value = String::from("../")
                        }
                    }
                    "windows" => {
                        if value.eq(&mut ".") {
                            *value = String::from(".\\")
                        }
                        if value.eq(&mut "..") {
                            *value = String::from("..\\")
                        }
                    }
                    _ => panic!("Incorrect path"),
                }
            }
        }
        self.request_items.reverse();
    }

    fn correct_path(&mut self) {
        let mut counter = 0;
        if self.request_items.is_empty() {
            self.way_step_by_step();
        }
        for user_i in 0..self.request_items.len() {
            let mut possible_directories = directory_contents(self.request_items[user_i].as_str());
            for pos_j in 0..possible_directories.len() {
                if self
                    .request_items
                    .get(user_i + 1)
                    .unwrap_or(&self.request_items.get(user_i).unwrap().to_lowercase())
                    .to_lowercase()
                    == possible_directories[pos_j].to_lowercase()
                {
                    self.request_items[user_i + 1] = possible_directories.remove(pos_j);
                    counter += 1;
                    break;
                }
            }
        }
        if Path::new(self.request_items.last().unwrap()).exists() {
            self.correct_path = PathBuf::from(self.request_items.last().unwrap());
        } else if let "linux" | "macos" = self.os {
            if Path::new(&self.request_items[counter]).exists() && counter != 0 {
                self.correct_path = PathBuf::from(format!(
                    "{}{}",
                    self.request_items[counter],
                    self.request_items
                        .last()
                        .unwrap()
                        .split_at(self.request_items[counter].len())
                        .1
                ));
            }
        }
    }
    /// returns the real path ([`&PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) if the real path is found
    /// but if not, it returns the path you originally entered.\
    /// To create files/folders in the new path use:
    /// - [`auto_write()`](<struct.QFilePath.html#method.auto_write>)
    /// - [`write_only_new()`](<struct.QFilePath.html#method.write_only_new>)
    /// # Example
    /// ```
    /// use qfile::*;
    /// use std::path::PathBuf;
    /// # fn main() {
    ///     // The file already exists
    ///     // The real file path: "./My_First_Folder/New_File.txt"
    ///     let mut file = QFilePath::add_path("my_first_Folder/new_file.txt").unwrap();
    ///     assert_eq!(
    ///         file.get_path_buf(),
    ///         &PathBuf::from("./My_First_Folder/New_File.txt")
    ///     );
    /// # }
    /// ```
    pub fn get_path_buf(&mut self) -> &PathBuf {
        match self.os {
            "linux" | "macos" => {
                if self.user_path.exists() {
                    if !self.correct_path.to_str().unwrap().is_empty() {
                        return &self.correct_path;
                    }
                    return &self.user_path;
                }
                if !self.update_path
                    && self.correct_path.to_str().unwrap().is_empty()
                    && self.user_path.to_str().unwrap() != self.correct_path.to_str().unwrap()
                {
                    self.correct_path();
                }
                if self.correct_path.to_str().unwrap().is_empty() {
                    return &self.user_path;
                }
                return &self.correct_path;
            }
            "windows" => {
                if !self.correct_path.exists() {
                    self.correct_path();
                    if !self.correct_path.to_str().unwrap().is_empty() && self.update_path {
                        let temp = self.request_items.pop();
                        let mut last = String::new();
                        if self.request_items.last().unwrap() != ".\\"
                            && !self.request_items.last().unwrap().contains(":\\")
                            && !self.request_items.last().unwrap().contains("..\\")
                        {
                            last = format!(
                                "{}\\{}",
                                self.request_items.pop().unwrap(),
                                self.file_name.to_str().unwrap()
                            );
                        } else {
                            last = temp.unwrap();
                        }
                        self.correct_path = PathBuf::from(last);
                        return &self.correct_path;
                    }
                }
                if !self.correct_path.to_str().unwrap().is_empty() {
                    if self.update_path {
                        self.correct_path();
                    }
                    return &self.correct_path;
                }
                return &self.user_path;
            }
            _ => panic!("Incorrect path"),
        }
    }

    /// returns the real path (`&str`) if the real path is found
    /// but if not, it returns the path you originally entered.\
    /// To create files/folders in the new path use:
    /// - [`auto_write()`](<struct.QFilePath.html#method.auto_write>)
    /// - [`write_only_new()`](<struct.QFilePath.html#method.write_only_new>)

    /// # Example
    /// ```
    /// use qfile::QFilePath;
    /// use std::path::PathBuf;
    /// # fn main() {
    ///     // The file already exists
    ///     // The real file path: "./My_First_Folder/New_File.txt"
    ///     let mut file = QFilePath::add_path("my_first_Folder/new_file.txt");
    ///     assert_eq!(file.get_path_str(), "./My_First_Folder/New_File.txt");
    /// # }
    /// ```
    pub fn get_path_str(&mut self) -> &str {
        self.get_path_buf().to_str().unwrap()
    }

    /// If the file exists, it returns the [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) with the specified permissions:
    /// - read only
    /// - write only
    /// - read and write
    ///
    /// ( this method does not set permissions on files on your system, it returns an already opened file (RW)
    /// with specific permissions for the code)
    /// # Example
    /// ```
    /// use qfile::*;
    /// use std::io::BufReader;
    /// use std::io::Read;
    /// # fn main() {
    ///     // The file already exists
    ///     // The real file path: "./My_First_Folder/New_File.txt"
    ///     // File content: Hello World
    ///     let file = QFilePath::add_path("my_first_Folder/new_file.txt")
    ///         .unwrap()
    ///         .get_file(Permissions::RW)
    ///         .unwrap();
    ///     let mut buffer = Vec::new();
    ///     // Read file into vector.
    ///     BufReader::new(file).read_to_end(&mut buffer).unwrap();
    ///     // Show result
    ///     assert_eq!(
    ///         buffer,
    ///         vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
    ///     )
    /// # }
    /// ```
    pub fn get_file(&mut self, permission: Permissions) -> Result<File, io::Error> {
        let rs = self.get_path_buf();
        match return_file(rs.to_str().unwrap()) {
            Ok(_) => match permission {
                Permissions::R => Ok(OpenOptions::new().read(true).write(false).open(rs).unwrap()),
                Permissions::W => Ok(OpenOptions::new().read(false).write(true).open(rs).unwrap()),
                Permissions::RW => Ok(OpenOptions::new().read(true).write(true).open(rs).unwrap()),
            },
            Err(err) => return Err(err),
        }
    }
}
impl<'a> Drop for QFilePath<'a> {
    fn drop(&mut self) {}
}
fn return_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => Err(err),
    }
}
fn directory_contents(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(paths) = fs::read_dir(path) {
        for items in paths {
            if let Ok(items) = items {
                files.push(items.path().display().to_string());
            }
        }
    }
    return files;
}
