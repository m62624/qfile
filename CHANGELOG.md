# Changelog
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