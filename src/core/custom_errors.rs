use thiserror::Error;
#[derive(Error, Debug)]
pub enum QPathError {
    #[error("You are using the windows path format for Unix. Use `unix` format for the path:\n> ./folder1/folder2/file.txt\n> ../folder2/file.txt\n> ./file.txt")]
    UnixPathIncorrect,
    #[error("You are using the unix path format for Windows. Use `windows` format for the path:\n> .\\folder1\\folder2\\file.txt\n> ..\\folder2\\file.txt\n> .\\file.txt")]
    WindowsPathIncorrect,
    #[error("SystemNotDefined (you can implement a pattern for an undefined OS - `TargetOS`)")]
    SystemNotDefined,
    #[error("The path is empty")]
    PathIsEmpty,
}
