use thiserror::Error;
#[derive(Error, Debug)]
/// Error type for handling QFilePath cases
pub enum QPackError {
    /// Returns an error if you use a **non-unix** format for the path
    #[error("You are using the windows path format for Unix. Use `unix` format for the path:\n> ./folder1/folder2/file.txt\n> ../folder2/file.txt\n> ./file.txt")]
    UnixPathIsIncorrect,
    /// Returns an error if you use a **non-windows** format for the path
    #[error("You are using the unix path format for Windows. Use `windows` format for the path:\n> .\\folder1\\folder2\\file.txt\n> ..\\folder2\\file.txt\n> .\\file.txt")]
    WindowsPathIsIncorrect,
    /// Returns an error if the library is not prepared for this operating system
    #[error("SystemNotDefined")]
    SystemNotDefined,
    /// Returns an error if you specify an empty path
    #[error("The path is empty")]
    PathIsEmpty,
    #[error("The path is incorrect")]
    PathIsIncorrect,
    /// Returns an error if you try to get `QPackError` from `Box<dyn Error>` that contains error != `QPackError`.
    #[error("Not covered error")]
    NotQPackError,
    #[error("Asynchronous call from SyncPack (use a similar function from SyncPack)")]
    AsyncCallFromSync,
    #[error("Synchronous call from AsyncPack (use a similar function from SyncPack)")]
    SyncCallFromAsync,
    #[error("Error from IO")]
    IoError(#[from] std::io::Error),
}
