
[![Crate](https://img.shields.io/crates/v/qfile?color=green)](https://crates.io/crates/qfile)
[![Docs](https://img.shields.io/docsrs/qfile)](https://docs.rs/qfile/latest/qfile/)
[![Changelog](https://img.shields.io/badge/changelog-qfile-blue)](https://github.com/m62624/qfile/blob/main/CHANGELOG.md)

# Qfile




Qfile is a file management crate. Supports synchronous and asynchronous file operations.

- File search
- Read, Write 

Methods to read, write, get the right path are case insensitive

---
# Paths syntax

## Linux 
  
```rust
    let path1 = "File.txt";
    let path2 = "./File.txt";
    let path3 = "../../File.txt";
    let path4 = String::from("Folder/Folder/File.txt");
```

## Windows 
  
```rust
    let path1 = "File.txt";
    let path2 = ".\\File.txt";
    let path3 = "..\\..\\File.txt";
    let path4 = "D:\\Folder\\file.txt";
    let path5 = r"D:\Folder\file.txt";
    let path6 = String::from("D:\\Folder\\file.txt");
```

---
# Writing to a file

Creates or opens if a file exists (case insensitive)

### Example (Sync code)
```rust
use qfile::{sync_qfile::TraitQFileSync, QFilePath};
use std::error::Error;
fn example() -> Result<(), Box<dyn Error>> {
    QFilePath::add_path("Folder/File.txt")?.auto_write("text text text")
}
```

### Example (Async code)
```rust
use qfile::{async_qfile::TraitQFileAsync, QFilePath};
use std::error::Error;
async fn example() -> Result<(), Box<dyn Error + Send + Sync>> {
    QFilePath::add_path_for_async("Folder/File.txt")?
        .lock()
        .await
        .async_auto_write("text text text")
        .await
} 

```
### Linux & Windows

 - If the path exists, we work with the file

 |                            | Linux                        | Windows                      |
 | -------------------------- | ---------------------------- | ---------------------------- |
 | **The path we specified**: | `folder1/FolDER2/file.TXT`   | `folder1\FolDER2\file.TXT`   |
 | **Real path** :            | `./Folder1/Folder2/file.txt` | `.\Folder1\Folder2\file.txt` |
 | **Result** :               | `./Folder1/Folder2/file.txt` | `.\Folder1\Folder2\file.txt` |

 - If the file/path is not found, creates a new path with the file

 |                            | Linux                               | Windows                             |
 | -------------------------- | ----------------------------------- | ----------------------------------- |
 | **The path we specified**: | `./main_folder/folder_new/file.txt` | `.\main_folder\folder_new\file.txt` |
 | **Real path** :            | `./Main_Folder`                     | `.\Main_Folder`                     |
 | **Result** :               | `./Main_Folder/folder_new/file.txt` | `.\Main_Folder\folder_new\file.txt` |
 
 > * The Windows file system treats file and directory names as **case insensitive**. `file.txt` and `FILE.txt` will be treated as equivalent files (Although the path is **case insensitive** in windows (`..\FOLDER\file.txt`), you can return a **case-sensitive** path with : `get_path_string()` or `get_path_buf()`).

---
# Reading a file

Gets the string from the file (case insensitive)

### Example (Sync code)
```rust
use qfile::{sync_qfile::*,QFilePath};
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let mut file = QFilePath::add_path("file.txt")?;
    let text = file.read()?;
    assert_eq!(text, ":D :D :D");
    Ok(())
}
```

### Example (Async code)
```rust
use qfile::{async_qfile::*, QFilePath};
use std::sync::Arc;
use std::thread;
use futures::executor::block_on;
fn main() {
    let file = QFilePath::add_path_for_async("Folder/File.txt").unwrap();
    // some code
    let file_arc = Arc::clone(&file);
    let joinhandle = thread::spawn(|| {
        block_on(async move { file_arc.lock().await.async_read().await.unwrap() })
    });
    assert_eq!(joinhandle.join().unwrap(), ":D :D :D");
}
```

---
# Paths finder

If the full path is unknown, we can get a list of all matching paths (case insensitive)

### Example (Sync code)
```rust
use std::sync::mpsc;
use qfile::{sync_qfile::*,QFilePath};
fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();
    QFilePath::find_paths(Directory::Everywhere, "file.txt", false, tx)?;
    // some code
    while let Ok(path) = rx.recv() {
        println!("{}", path.display());
    }
    Ok(())
}

```
Output:
> - /home/userFolder/.local/share/Trash/files/FOLDER/new_folder/file.txt/file.txt
> - /home/userFolder/.local/share/Trash/files/.56.Polygon/file.txt
> - /home/userFolder/.local/share/Trash/files/.46.Polygon/file.txt
> - /home/userFolder/.local/share/Trash/files/.10.Polygon/File.txt
> - /home/userFolder/.local/share/Trash/files/.50.Polygon/file.txt

### Exmple (Async code)
```rust
use futures::executor;
use qfile::{
    async_qfile::{
        AsyncChannel::{self, SendError, Sender},
        AsyncPathModule::PathBuf,
        Directory, TraitQFileAsync,
    },
    QFilePath,
};
use std::thread;
async fn example(sender: Sender<PathBuf>) -> Result<(), SendError<PathBuf>> {
    QFilePath::async_find_paths(Directory::Everywhere, "main.rs", false, sender).await
}

fn main() {
    let (tx, rx) = AsyncChannel::unbounded();
    let jh = thread::spawn(move || {
        executor::block_on(async {
            if let Ok(_) = example(tx).await {
                while let Ok(path) = rx.recv().await {
                    println!("{}", path.display());
                }
            }
        });
        //some code
    });
    jh.join().unwrap();
}
```
Output:
> - /home/userFolder/.local/share/Trash/files/main.rs
> - /home/userFolder/.local/share/Trash/info/main.rs.trashinfo
> - /home/userFolder/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/backtrace/tests/accuracy/main.rs
> - /home/userFolder/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/share/doc/rust/html/error_codes/main.rs

---

 # Changelog
 [List](https://github.com/m62624/qfile/blob/main/CHANGELOG.md)
 # License
 [MIT](https://choosealicense.com/licenses/mit/)
