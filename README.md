
[![Crate](https://img.shields.io/crates/v/qfile?color=green)](https://crates.io/crates/qfile)
[![Docrs](https://img.shields.io/crates/v/qfile?color=blue&label=docs)](https://docs.rs/qfile/1.0.0/qfile/)

 # Qfile

 Crate for accessing a file by path, case insensitive. Automatic detection, create a path with a new file or open an existing file.

 # Examples
```rust
    //add_path()
    //Constructor for adding a file path. 
    //After using the write() or read() methods, and if Ok(),
    //we get the correct path, which will be used as a cache when we reuse
   
    // the real file path: `./FOLDER/folder/File.txt`
    let mut file = QFilePack::add_path("./folder/folder/file.txt");
    {
       // The real path is searched after the first method call. 
       // It's stored in the structure
       file.write("text_1").unwrap();
    }
    // we get the saved path right away
    file.write("text_2").unwrap();
    println!("{}",file.read().unwrap());

    //output: text_1text2
  
```

```rust
use qfile::QFilePack;
use std::fs::File;

    // the real file path: `./new_FILE.txt`
    let mut file = QFilePack::add_path("./new_file.txt").file().unwrap();
    
    // Get the file directly
    // You can use the function to retrieve data 
    // in bytes format or use it for any other option
    assert_eq!(file.metadata().unwrap().is_file(), true);
```

---

# Paths syntax
  - linux & macos (**doesn't work** with files with '/', "x/y/z.txt" in the name on macos)
  > `./folder/folder/file.txt`
  - Windows 
  > `.\\folder\\folder\\file.txt`\
  > `D:\\folder\\folder\\file.txt`

# Auto Mode

Creates a new path with file. Writes new data to an empty file
### Example
```rust
    let mut file = QFilePack::add_path("./new_file.txt");
    {
        file.write(":D").unwrap();
    }
    file.write(":D").unwrap();
    assert_eq!(file.read().unwrap(),":D:D");
    
```
 - If the path exists, regardless of the case, we work with the file
 
 > **The path we specified**: `./FLDR/FlDr/file.TXT`\
  **real path** : `./fldr/fldr/file.txt`\
  **Result** : `"./fldr/fldr/file.txt"`

- If the file/path is not found, creates a new path with the file (*if initial path exists*)
 
 > **The path we specified**: `./fldr/fldr_new/file.txt`\
  **real path** : `./fldr`\
  **Result** : `./fldr/fldr_new/file.txt`
 
  but if the initial path is case different, then a *new path with the file* is created 
 
 > **The path we specified**: `./FLDR/fldr_new/file.TXT`\
  **real path** : `./fldr`\
  **Result** :\
  `./fldr`\
  `./FLDR/fldr_new/file.TXT`

 # License
 [MIT](https://choosealicense.com/licenses/mit/)