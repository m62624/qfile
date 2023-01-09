mod read;
mod write;
use crate::dpds_path::{fs, io, lazy_static, File, OpenOptions, Path, Regex, __Deref, env};
#[derive(Debug)]
pub enum Flag {
    New,
    Auto,
    Old,
}
/// Enumeration for selecting read and write permissions
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
pub struct QFilePath<'a> {
    request_items: Vec<String>,
    //================
    user_path: &'a str,
    file_name: &'a str,
    correct_path: String,
    //================
    os: &'a str,
    flag: Flag,
    update_path: bool,
}

//======================================================
impl<'a> QFilePath<'a> {
    /// Constructor for adding a file path.\
    /// After using the [`auto_write()`](<struct.QFilePath.html#method.auto_write>) or [`read()`](<struct.QFilePath.html#method.read>) methods (also the [`cache_path()`](struct.QFilePath.html#method.cache_path) if the path exists), and if `Ok`,\
    /// we get the correct path, which will be used as a cache when we reuse
    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    ///
    /// // the real file path: `./FOLder/Folder/NEW.txt`
    /// let mut file = QFilePath::add_path("./folder/Folder/new.txt");
    /// // The real path is searched after the first method call
    /// // (it's stored in the structure).
    /// file.auto_write("Olddata").unwrap();
    /// // we get the saved path right away
    /// file.auto_write("Newdata").unwrap();
    /// assert_eq!(file.read().unwrap(), "OlddataNewdata");
    ///
    /// # }
    /// ```
    pub fn add_path(path: &'a str) -> Self {
        QFilePath {
            request_items: Default::default(),
            user_path: path,
            file_name: Default::default(),
            correct_path: Default::default(),
            os: env::consts::OS,
            flag: Flag::Auto,
            update_path: false,
        }
    }
    fn way_step_by_step(&mut self) {
        let mut items = |rgx: &Regex, path: &str| {
            let (mut folders, mut i) = (Vec::new(), 1);
            let mut temp = String::from(path);
            match self.os {
                "linux" | "macos" => {
                    lazy_static! {
                        static ref SL: Regex = Regex::new(r"^/|^../|^./").unwrap();
                    }
                    if !SL.is_match(&temp) {
                        temp = format!("./{}", temp);
                    }
                }
                "windows" => {
                    lazy_static! {
                        static ref SL: Regex = Regex::new(r"^.:\\+|^..\\|^.\\").unwrap();
                    }
                    if !SL.is_match(&temp) {
                        temp = format!(".\\{}", temp);
                    }
                }
                _ => {
                    panic!(":: unsupported system ::")
                }
            }
            let mut captures = rgx.captures_iter(&temp);
            folders.push(captures.next().unwrap()[0].to_string());
            for element in captures {
                folders.push(format!("{}{}", folders[i - 1], &element[0]));
                i += 1;
            }
            self.request_items = folders;
        };

        match self.os {
            "linux" | "macos" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"/[^/]+|../|./|[^/]+").unwrap();
                }
                return items(RE.deref(), self.user_path);
            }
            "windows" => {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r" ^.:\\+|^..\\|^.\\|.+?[^\\]+").unwrap();
                }
                return items(RE.deref(), self.user_path);
            }
            _ => {
                panic!(":: unsupported system ::")
            }
        };
    }

    fn correct_path(&mut self) {
        self.way_step_by_step();
        let request_items = &mut self.request_items;
        for user_i in 0..request_items.len() {
            let mut possible_directories = directory_contents(request_items[user_i].as_str());
            for pos_j in 0..possible_directories.len() {
                if request_items
                    .get(user_i + 1)
                    .unwrap_or(&request_items.get(user_i).unwrap().to_lowercase())
                    .to_lowercase()
                    == possible_directories[pos_j].to_lowercase()
                {
                    request_items[user_i + 1] = possible_directories.remove(pos_j);
                    break;
                }
            }
        }
        let result = request_items.last();
        if Path::new(result.unwrap()).exists() {
            self.correct_path = result.unwrap().to_string();
            self.request_items.clear();
            self.request_items.shrink_to_fit();
        }
    }
    /// returns the real path if the real path is found
    /// but if not, it returns the path you originally entered.
    /// To create files/folders in the new path use:
    /// - [`auto_write()`](<struct.QFilePath.html#method.auto_write>)
    /// - [`write_only_new()`](<struct.QFilePath.html#method.write_only_new>)

    /// # Example
    /// ```
    /// # use qfile::QFilePath;
    /// # fn main() {
    ///
    /// // The file already exists
    /// // The real file path: "./My_First_Folder/New_File.txt"
    /// let mut file = QFilePath::add_path("my_first_Folder/new_file.txt");
    /// assert_eq!(file.cache_path(),"./My_First_Folder/New_File.txt");
    ///
    /// # }
    /// ```
    pub fn cache_path(&mut self) -> &str {
        if let true = Path::new(self.user_path).exists() {
            if !self.correct_path.is_empty() && self.user_path != self.correct_path {
                return self.correct_path.as_str();
            }
            if self.os != "windows" {
                return self.user_path;
            }
            self.correct_path();
            return self.correct_path.as_str();
        }
        if self.correct_path.is_empty() {
            self.correct_path();
            if !self.correct_path.is_empty() {
                return self.correct_path.as_str();
            }
        }
        if let true = Path::new(self.correct_path.as_str()).exists() {
            return self.correct_path.as_str();
        }
        return self.user_path;
    }

    /// If the file exists, it returns the [`File`](https://doc.rust-lang.org/std/fs/struct.File.html) with the specified permissions:
    /// - read only
    /// - auto_write only
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
    ///     let file = QFilePath::add_path("my_first_Folder/new_file.txt").get_file(Permissions::RW).unwrap();
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
        let rs = self.cache_path();
        match return_file(rs) {
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
