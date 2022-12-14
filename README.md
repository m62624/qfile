
[![Crate](https://img.shields.io/crates/v/qfile?color=green)](https://crates.io/crates/qfile)
[![Docrs](https://img.shields.io/crates/v/qfile?color=blue&label=docs)](https://docs.rs/qfile/1.0.0/qfile/)

 # Qfile

 Crate for accessing a file by path, case insensitive

 # Example
 ```
 use qfile::*;
fn main() {
    let mut file = QFilePack::add_path("./Folder1/Folder2/file.txt");
    file.write("text_1").unwrap();
    let data = file.read().unwrap();
    println!("{}",data);
}
```
# Paths syntax
  - Windows 
  > `".\\folder\\folder\\file.txt"`\
  > `"folder\\folder\\file.txt"`\
  > `D:\\"folder\\folder\\file.txt"`
  - linux
  > `"./folder/folder/file.txt"`\
  > `"folder/folder/file.txt"`
  - macos   (**doesn't work** with files with '/', "x/y/z.txt" in the name on macos)
  > `"./folder/folder/file.txt"`

# Flag mode

## New mode
Creates a new path with file. Writes new data to an empty file
### Example
    ```
    let mut file = QFilePack::add_path("./new_file.txt");
    file.write(":D").unwrap();
    assert_eq!(file.read().unwrap(),":D");
    
    ```
## Auto mode 
 - If the path exists, regardless of the case, we work with the file
 
 > **The path we specified**: `"./Folder1/folDER2/file.TXT"`\
  **real path** : `"./Folder1/Folder2/file.txt"`\
  **Result** : `"./Folder1/Folder2/file.txt"`

- If the file/path is not found, creates a new path with the file (*if initial path exists*)
 
 > **The path we specified**: `"./Folder1/newFolder/file.TXT"`\
  **real path** : `"./Folder1/newFolder/file.txt"`\
  **Result** : `"./Folder1/newFolder/file.txt"`
 
  but if the initial path is case different, then a *new path with the file* is created 
 
 > **The path we specified**: `"./folder1/newFolder/file.TXT"`\
  **real path** : `"./folder1/newFolder/file.txt"`\
  **Result** : `"./folder1/newFolder/file.txt"`

 # License
 [MIT](https://choosealicense.com/licenses/mit/)