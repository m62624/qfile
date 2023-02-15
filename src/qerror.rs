use super::CodeStatus;
use super::QFilePath;
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
    /// Returns an error if you try to get `QPackError` from `Box<dyn Error>` that contains error != `QPackError`.
    #[error("Not covered error")]
    NotQPackError,
    #[error("Asynchronous call from SyncPack (use a similar function from SyncPack)")]
    AsyncCallFromSync,
    #[error("Synchronous call from AsyncPack (use a similar function from SyncPack)")]
    SyncCallFromAsync,
}
impl QFilePath {
    fn check_status_code(&self, status: CodeStatus) -> Result<(), QPackError> {
        match self.status_mod {
            CodeStatus::SyncStatus => {
                if self.status_mod == status {
                    return Ok(());
                }
                return Err(QPackError::AsyncCallFromSync);
            }
            CodeStatus::AsyncStatus => {
                if self.status_mod == status {
                    return Ok(());
                }
                return Err(QPackError::SyncCallFromAsync);
            }
        }
    }
}
