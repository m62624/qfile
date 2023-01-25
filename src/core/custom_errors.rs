use super::{Arc, AsyncMutex, QFilePath};
use thiserror::Error;
#[derive(Error, Debug)]
pub enum QPackError {
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
    #[error("Async context error")]
    AsyncIOError(AsyncIO),
    #[error("Sync context error")]
    SyncIOError(SyncIO),
    #[error("Not covered error")]
    NotQPackError,
}
#[derive(Error, Debug)]
pub enum AsyncIO {
    #[error("Async Error from IO")]
    IO(#[from] async_std::io::Error),
}
#[derive(Error, Debug)]
pub enum SyncIO {
    #[error("Sync Error from IO")]
    IO(#[from] std::io::Error),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for QPackError {
    fn from(value: Box<dyn std::error::Error + Send + Sync>) -> Self {
        if let Ok(unpacked_value) = value.downcast::<QPackError>() {
            return *unpacked_value;
        }
        QPackError::NotQPackError
    }
}
impl From<Result<Arc<AsyncMutex<QFilePath>>, Box<dyn std::error::Error + Send + Sync>>>
    for QPackError
{
    fn from(
        value: Result<Arc<AsyncMutex<QFilePath>>, Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        QPackError::from(value.err().unwrap())
    }
}
