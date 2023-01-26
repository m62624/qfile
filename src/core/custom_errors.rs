use super::{Arc, AsyncMutex, QFilePath};
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
    /// Custom error for capturing `async IO` errors
    #[error("Async context error")]
    AsyncIOError(AsyncIO),
    /// Custom error for capturing `sync IO` errors
    #[error("Sync context error")]
    SyncIOError(SyncIO),
    /// Returns an error if you try to get `QPackError` from `Box<dyn Error>` that contains error != `QPackError`.
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
impl From<Box<dyn std::error::Error>> for QPackError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
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
impl From<Result<QFilePath, Box<dyn std::error::Error>>> for QPackError {
    fn from(value: Result<QFilePath, Box<dyn std::error::Error>>) -> Self {
        QPackError::from(value.err().unwrap())
    }
}
//===========================================================================

impl<'a> From<&'a Box<dyn std::error::Error>> for &'a QPackError {
    fn from(value: &Box<dyn std::error::Error>) -> &QPackError {
        if let Some(unpacked_value) = value.downcast_ref::<QPackError>() {
            return unpacked_value;
        }
        &QPackError::NotQPackError
    }
}
