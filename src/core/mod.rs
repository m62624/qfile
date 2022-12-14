mod read;
mod write;
use crate::dpds_path::{fs, io, lazy_static, ErrorKind, File, Path, Regex, __Deref};
use std::env;
#[derive(Debug)]
pub enum Flag {
    New,
    Auto,
    Old,
}
#[derive(Debug)]
/// A structure for storing the file path\
/// 
///  The structure stores :
/// - true file path (**used as a [cache](<struct.QFilePack.html#method.add_path>) for reuse**)
/// - possible file paths
/// - file name
/// - os (information about what format to look for the file `/` and `\\`)
pub struct QFilePack<'a> {
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
impl<'a> QFilePack<'a> {
    ///  **The constructor for the cache**. Constructor for adding a file path. After using the `write()` or `read()` methods, and if Ok(), we get the correct path, which will be used as a cache when we reuse
    ///
    /// # Example
    /// ```
    /// use qfile::QFilePack;
    /// # fn main() {
    ///     //---
    ///     // the real file path:
    ///     // ./FOLder/Folder/NEW.txt
    ///     let path = "./folder/Folder/new.txt";
    ///     let mut file = QFilePack::add_path(path);
    ///     {
    ///     // The real path is searched after the first method call. The real path is stored in the structure
    ///     file.write("Oldata").unwrap();
    ///     }
    ///     // we get the saved path right away
    ///     file.write("Newdata").unwrap();
    ///     assert_eq!(file.read().unwrap(), "OldataNewdata");
    ///     //---
    /// # }
    ///```
    ///
    pub fn add_path(path: &'a str) -> Self {
        QFilePack {
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
            let mut captures = rgx.captures_iter(path);
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
                    static ref RE: Regex =
                        Regex::new(r"(?:\./|\.\.|(?:\.\./|\./|[\./])?[^/]*)").unwrap();
                }
                return items(RE.deref(), self.user_path);
            }
            "windows" => {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r"(?:.:\\|\.\\|\.\.|(?:\.\.\\|\.\\|[\.\\])?[^\\]*)").unwrap();
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
            // println!("{}", request_items[user_i]);
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
        }
    }
    fn cache_path(&mut self) -> &str {
        if Path::new(self.user_path).exists() {
            if !self.correct_path.is_empty() && self.user_path != self.correct_path {
                return self.correct_path.as_str();
            }
            self.user_path
        } else if self.correct_path.is_empty() {
            self.correct_path();
            if self.correct_path.is_empty() {
                return self.user_path;
            }
            self.correct_path.as_str()
        } else {
            // self.user_path
            self.correct_path.as_str()
        }
    }

    /// Get the file directly\
    /// You can use the function to retrieve data in byte format or in some other way
    /// # Example
    /// ```
    /// use qfile::*;
    /// use std::fs::File;
    /// # fn main(){
    /// //---
    /// // the real file path:
    /// // ./new_FILE.txt
    /// let mut qpack = QFilePack::add_path("./new_file.txt");
    /// let file = qpack.file().unwrap();
    /// assert_eq!(file.metadata().unwrap().is_file(), true);
    /// //---
    /// # }
    /// ```
    pub fn file(&mut self) -> Result<File, io::Error> {
        match get_file(self.cache_path()) {
            Ok(fl) => Ok(fl),
            Err(err) => return Err(err),
        }
    }
}
//======================================================
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err),
            ErrorKind::PermissionDenied => Err(err),
            ErrorKind::InvalidData => Err(err),
            _ => panic!(":: other errors ::"),
        },
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
    // self.possible_directories = files;
    return files;
}

//=====================================(tests)=====================================
#[cfg(target_family = "unix")]
#[test]
fn test_way_step_by_step() {
    let mut temp = QFilePack::add_path("./Polygon/Don't delete/test-1.txt");
    temp.way_step_by_step();
    assert_eq!(
        temp.request_items,
        vec![
            "./",
            "./Polygon",
            "./Polygon/Don't delete",
            "./Polygon/Don't delete/test-1.txt"
        ]
    );
}
#[cfg(target_family = "unix")]
#[test]
fn test_path_content() {
    dbg!(directory_contents("./Polygon/Don't delete"));
    assert_eq!(
        directory_contents("./Polygon/Don't delete"),
        vec![
            "./Polygon/Don't delete/test-1.txt",
            "./Polygon/Don't delete/temp3.txt",
            "./Polygon/Don't delete/temp1.txt",
            "./Polygon/Don't delete/tempFolder",
            "./Polygon/Don't delete/temp2.txt",
        ]
    )
}
#[cfg(target_family = "unix")]
#[test]
fn test_correct_path_1() {
    let mut temp = QFilePack::add_path("./polygon/Read/test-1.txt");
    temp.correct_path();
    dbg!(temp);
    assert_eq!(true, true);
}
#[cfg(target_family = "unix")]
#[test]
fn test_correct_path_2() {
    let mut temp = QFilePack::add_path("./polygon/READ/TEst-2.txt");
    temp.correct_path();
    assert_eq!(temp.correct_path, "./Polygon/Read/TESt-2.txt");
}
#[cfg(target_family = "unix")]
#[test]
fn test_correct_path_3() {
    let mut temp = QFilePack::add_path("./polygon/does_not_exist.txt");
    temp.correct_path();
    assert_eq!(temp.correct_path, "");
}
