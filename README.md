
---
[![Crate](https://img.shields.io/badge/crates.io-v0.1.2-green)](https://crates.io/crates/qfile)
[![Docrs](https://img.shields.io/docsrs/qfile/0.1.2?color=blue&logo=doc)](https://docs.rs/qfile/0.1.2/qfile/)

The crate for working with files without taking into account the case of the path.
Automatic detection, create a path with a new file or open an existing file.

 # Usage
 Add this to your Cargo.toml:
```toml
[dependencies]
qfile="0.1.3"
```
 # Example
 ```
 use qfile::{file_read, file_write, Flag};
fn main() {
    file_write(
        "./Folder1/NewFolder1/file_new.txt",
        "TEXT TEXT TEXT",
        Flag::Auto,
    )
    .unwrap();
    println!("{}",file_read("./Folder1/NewFolder1/file_new.txt").unwrap());
}
 ```

  # Paths syntax
  - Windows 
  > `".\\folder\\\folder\\file.txt"`
  - linux
 > `"./folder/folder/file.txt"`
  - macos   (**doesn't work** with files with '/', "x/y/z.txt" in the name on macos)
 > `"./folder/folder/file.txt"`

# Flag mode

## New mode
Creates a new path with file. Writes new data to an empty file
### Example
    ```
      let path = "./Folder1/NewFolder1/file_new.txt";
      
      assert_eq!(file_write(path, "ok", Flag::New).unwrap(), file_read(path).unwrap());
    ```
## Auto mode 
 - If the path exists, regardless of the case, we work with the file `(Flag::Old)`
 
 > **The path we specified**: `"/Folder1/folDER2/file.TXT"`\
  **real path** : `"/Folder1/Folder2/file.txt"`\
  **Result** : `"/Folder1/Folder2/file.txt"`

- If the file/path is not found, creates a new path with the file (*if initial path exists*) `(Flag::New)`
 
 > **The path we specified**: `"/Folder1/newFolder/file.TXT"`\
  **real path** : `"/Folder1/newFolder/file.txt"`\
  **Result** : `"/Folder1/newFolder/file.txt"`
 
  but if the initial path is case different, then a *new path with the file* is created `(Flag::New)`
 
 > **The path we specified**: `"/folder1/newFolder/file.TXT"`\
  **real path** : `"/folder1/newFolder/file.txt"`\
  **Result** : `"/folder1/newFolder/file.txt"`
  ### Example
  ```
    let path = "./Folder1/not_existing_folder/file_new.txt";
    
    assert_eq!(file_write(path, "ok", Flag::Auto).unwrap(), file_read(path).unwrap());
  ```
## Old mode
 Finds an already existing file. Appends new data to an existing file
### Example
     ```
       let path = "./Folder1/NewFolder1/file_new.txt";
       
       assert_eq!(file_write(path, "ok", Flag::Old).unwrap(), file_read(path).unwrap());
     ```
 # License
 [MIT](https://choosealicense.com/licenses/mit/)

