# Changelog
## [2.1.0] -2023.01.11
### Changed
- `add_path()` returns `Result<Self, OsPathError>`
### Added 
- Custom errors:
  - `UnixPathIncorrect`
  - `WindowsPathIncorrect`

If you catch these errors, you can get a message:

---
#### Windows
```rust
use qfile::*;
fn main() {
    let path = QFilePath::add_path("./folder/file.txt");
    if let Err(err) = path {
        println!("{err}");
    }
}
```
Output:
> You are using the unix path format for Windows. Use `windows` format for the path:\
> \> .\folder1\folder2\file.txt\
> \> ..\folder2\file.txt\
> \> .\file.txt

---
#### Linux
```rust
use qfile::*;
fn main() {
    let path = QFilePath::add_path(".\\folder\\file.txt");
    if let Err(err) = path {
        println!("{err}");
    }
}
```
Output:
> You are using the windows path format for Unix. Use `unix` format for the path\
> \> ./folder1/folder2/file.txt \
> \> ../folder2/file.txt\
> \> ./file.txt

---

## [2.0.0] - 2023.01.11
### Added
- `get_path_str` - returns [`PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html) in `&str` format
- New examples and descriptions of how naming files work
### Changed
- `cache_path` renamed to `get_path_buf`. Now `get_path_buf` returns [`PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html).
- API - Changed the name of the methods, for a better understanding of their work
- updated documentation

## [1.1.4] - 2023.01.08
### Fixed
Files and folders were created in different paths - fixed 
## [1.1.3] - 2023.01.08
### Changed 
Now it is not necessary to specify `./` at the beginning of the path (but you can still write it)

Example:

| Before                      | After                    |
| --------------------------- | ------------------------ |
| ./Folder/folder/file.txt    | Folder/folder/file.txt   |
| ./file.txt                  | file.txt                 |
| .\\Folder\\Folder\\file.txt | Folder\\Folder\\file.txt |