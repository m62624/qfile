use thiserror::Error;
#[derive(Error, Debug)]
pub enum QPathError {
    #[error("You are using the windows path format for Unix. Use `unix` format for the path:\n> ./folder1/folder2/file.txt\n> ../folder2/file.txt\n> ./file.txt")]
    UnixPathIsIncorrect,
    #[error("You are using the unix path format for Windows. Use `windows` format for the path:\n> .\\folder1\\folder2\\file.txt\n> ..\\folder2\\file.txt\n> .\\file.txt")]
    WindowsPathIsIncorrect,
    #[error("SystemNotDefined (you can implement a pattern for an undefined OS - `TargetOS`)")]
    SystemNotDefined,
    #[error("The path is empty")]
    PathIsEmpty,
    /// This option for custom pattern
    #[error("The path is incorrect")]
    PathIsIncorrect,
}
#[derive(Error, Debug)]
pub enum QOptionCode {
    #[error("called async call for synchronous code")]
    IncompatibleModeForSync,
    #[error("called synchronous call for asynchronous code")]
    IncompatibleModeForAsync,
}
