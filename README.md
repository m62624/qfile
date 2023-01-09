
[![Crate](https://img.shields.io/crates/v/qfile?color=green)](https://crates.io/crates/qfile)
[![Docs](https://img.shields.io/docsrs/qfile)](https://docs.rs/qfile/latest/qfile/)

 # Qfile

 Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.

 # Examples
```rust
    //add_path() - constructor for adding a file path. 
    //After using the write() or read() methods, and if Ok(),
    //we get the correct path, which will be used as a cache when we reuse
   
    // the real file path: `./FOLDER/folder/File.txt`
    let mut file = QFilePath::add_path("./folder/folder/file.txt");
    // The real path is searched after the first method call.
    // It's stored in the structure
    file.auto_write("text_1").unwrap();
    // we get the saved path right away
    file.auto_write("text_2").unwrap();
    assert_eq!(file.read().unwrap(), "text_1text_2");
  
```

```rust
    use qfile::*;
    use std::io::BufReader;
    use std::io::Read;
    // The file already exists
    // The real file path: "./My_First_Folder/New_File.txt"
    // File content: Hello World
    let file = QFilePath::add_path("my_first_Folder/new_file.txt").get_file(Permissions::RW).unwrap();
    let mut buffer = Vec::new();
    // Read file into vector.
    BufReader::new(file).read_to_end(&mut buffer).unwrap();
    // Show result
    assert_eq!(
        buffer,
        vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
    )
```

---

# Paths syntax
  - linux & macos (macos: doesn't work with files that have a slash in the name: `fi/le.txt`)
  > `folder/folder/file.txt`\
  > `./folder/folder/file.txt`
  - Windows 
  > `folder\\folder\\file.txt`\
  > `.\\folder\\folder\\file.txt`\
  > `D:\\folder\\folder\\file.txt`

# Auto Mode

Creates a new path with file. Writes new data to an empty file
### Example
```rust
    let mut file = QFilePath::add_path("./file.txt");
    file.auto_write("ok").unwrap();
    //=========
    //*code*
    //=========
    file.auto_write("ok").unwrap();
    assert_eq(file.read().unwrap(),"okok");
    
```
 - If the path exists, we work with the file (case insensitive)
 
 |                            |                              |
 | -------------------------- | ---------------------------- |
 | **The path we specified**: | `folder1/FolDER2/file.TXT`   |
 | **Real path** :            | `./Folder1/Folder2/file.txt` |
 | **Result** :               | `./Folder1/Folder2/file.txt` |
 - If the file/path is not found, creates a new path with the file (*if initial path exists*)
 
 |                            |                                |
 | -------------------------- | ------------------------------ |
 | **The path we specified**: | `./folder/folder_new/file.txt` |
 | **Real path** :            | `./folder`                     |
 | **Result** :               | `./folder/folder_new/file.txt` |
 - but if the initial path is different case of letters and a new file/folder is specified in the path, then a new path is created with the file
 
 |                            |                                                        |
 | -------------------------- | ------------------------------------------------------ |
 | **The path we specified**: | `./FOLDER/Folder_new/file.txt`                         |
 | **Real path** :            | `./folder`                                             |
 | **Result** :               | `./FOLDER/Folder_new/file.txt` - (**new created path**) |
 |                            | `./folder` - (**real path**) 


 # Changelog
 [List](https://github.com/m62624/qfile/blob/main/CHANGELOG.md)
 # License
 [MIT](https://choosealicense.com/licenses/mit/)
