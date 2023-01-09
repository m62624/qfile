# Changelog
## [2.0.0] - 2023.01.09
### Changed
- cache_path() - removed unnecessary conditions
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