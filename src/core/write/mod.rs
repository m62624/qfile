pub mod files;
use self::files::{collect_folder, correct_path};
use super::get_file;
use crate::dpds_path::io::{self, ErrorKind, Write};
use crate::dpds_path::{DirBuilder, File, OpenOptions};

/// File/path option, preferred mode **auto**
<<<<<<< HEAD
=======
///  # Paths syntax
///  - Windows
///  > `".\\folder\\\folder\\file.txt"`
///  - linux
/// > `"./folder/folder/file.txt"`
///  - macos   (**doesn't work** with files with '/', "x/y/z.txt" in the name on macos)
/// > `"./folder/folder/file.txt"`
>>>>>>> origin/main
pub enum Flag {
    ///Creates a new path with file. Writes new data to an empty file
    /// # Examples
    /// ```
    ///   let path = "./Folder1/NewFolder1/file_new.txt";
    ///   
    ///   assert_eq!(file_write(path, "ok", Flag::New).unwrap(), file_read(path).unwrap());
    /// ```
    New,
    /// Auto option
    ///- If the path exists, regardless of the case, we work with the file `(Flag::Old)`
    ///
    ///> **The path we specified**: `"/Folder1/folDER2/file.TXT"`\
    /// **real path** : `"/Folder1/Folder2/file.txt"`\
<<<<<<< HEAD
    /// **Result** : `"/Folder1/Folder2/file.txt"`\
=======
    /// **Result** : `"/Folder1/Folder2/file.txt"`
>>>>>>> origin/main
    /// - If the file/path is not found, creates a new path with the file (*if initial path exists*) `(Flag::New)`
    ///
    ///> **The path we specified**: `"/Folder1/newFolder/file.TXT"`\
    /// **real path** : `"/Folder1/newFolder/file.txt"`\
<<<<<<< HEAD
    /// **Result** : `"/Folder1/newFolder/file.txt"`\  
=======
    /// **Result** : `"/Folder1/newFolder/file.txt"`
>>>>>>> origin/main
    ///
    /// but if the initial path is case different, then a *new path with the file* is created `(Flag::New)`
    ///
    ///> **The path we specified**: `"/folder1/newFolder/file.TXT"`\
    /// **real path** : `"/folder1/newFolder/file.txt"`\
<<<<<<< HEAD
    /// **Result** : `"/folder1/newFolder/file.txt"`\  
=======
    /// **Result** : `"/folder1/newFolder/file.txt"`
>>>>>>> origin/main
    /// # Examples
    /// ```
    ///   let path = "./Folder1/not_existing_folder/file_new.txt";
    ///   
    ///   assert_eq!(file_write(path, "ok", Flag::Auto).unwrap(), file_read(path).unwrap());
    /// ```
    Auto,
    ///Finds an already existing file. Appends new data to an existing file
    ///  # Examples
    /// ```
    ///   let path = "./Folder1/NewFolder1/file_new.txt";
    ///   
    ///   assert_eq!(file_write(path, "ok", Flag::Old).unwrap(), file_read(path).unwrap());
    /// ```
    Old,
}
<<<<<<< HEAD
/// Function for reading a file with operating modes **`Flag`**
/// Writes data (look at the flag mode in [Flag](enum.Flag.html))
=======
/// The function to create paths/files and write data to files with modes  (look at the flag mode in [Flag](enum.Flag.html))
/// # Examples
/// ```
///  //linux
///   let path = "./Folder1/NewFolder1/file_new.txt";
///   assert_eq!(file_write(path, "ok", Flag::Auto).unwrap(), file_read(path).unwrap());
///  //macos
///   let path = "./Folder1/NewFolder1/file_new.txt";
///   assert_eq!(file_write(path, "ok", Flag::Auto).unwrap(), file_read(path).unwrap());
///  //windows
///   let path = "..\\Folder1\\NewFolder1\\file_new.txt";
///   assert_eq!(file_write(path, "ok", Flag::Auto).unwrap(), file_read(path).unwrap());
/// ```
>>>>>>> origin/main
pub fn file_write(path: &str, text: &str, flag: Flag) -> Result<(), io::Error> {
    match flag {
        Flag::Auto => match get_file(path) {
            Ok(_) => return file_write(path, text, Flag::Old),
            Err(_) => match correct_path(path) {
                Ok(new_path) => file_write(&new_path, text, Flag::Old),
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => Ok({
                        let mut temp = collect_folder(path);
                        let name = temp.pop().unwrap();
                        let mut xl = collect_folder(&name);
                        let name = xl.pop().unwrap();
                        let name = name.replace(&xl.pop().unwrap(), "");
                        let temp = temp.pop().unwrap();
                        let result = format!("{}{}", temp, name);
                        if let Err(_) = correct_path(&temp) {
                            DirBuilder::new().recursive(true).create(&temp).unwrap();
                            return file_write(&result, text, Flag::New);
                        } else {
                            let temp = correct_path(&temp).unwrap();
                            let result = format!("{}{}", temp, name);
                            file_write(&result, text, Flag::New).unwrap();
                        }
                    }),
                    ErrorKind::PermissionDenied => {
                        panic!("Permission Denied");
                    }
                    _ => panic!("other errors"),
                },
            },
        },
        Flag::New => match File::create(path) {
            Ok(_) => OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap()
                .write_all(text.as_bytes()),
            Err(err) => return Err(err),
        },
        Flag::Old => OpenOptions::new()
            // .write(true)
            .append(true)
            .open(path)
            .unwrap()
            .write_all(text.as_bytes()),
    }
}
