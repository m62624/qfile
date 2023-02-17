use super::{Flag, PathBuf, QFilePath, QPackError};
use crate::init::path_for_write::*;
use crate::init::work_with_elements::*;
use crate::paths::get_path::*;
use async_fs;
use async_recursion::async_recursion;
use futures_lite::AsyncWriteExt;
use std::error::Error;
use std::fs;
use std::io::Write;
pub mod write {
    use super::*;
    pub fn auto_write<T: AsRef<str>>(slf: &mut QFilePath, text: T) -> Result<(), QPackError> {
        if slf.update_path {
            slf.correct_path = PathBuf::from(format!(
                "{}{}",
                slf.user_path.to_str().unwrap(),
                slf.file_name.to_str().unwrap()
            ));
        }
        match slf.flag {
            Flag::Old => {
                let temp = get_path_buf(slf)?;
                slf.flag = Flag::Auto;
                fs::OpenOptions::new()
                    .append(true)
                    .open(temp)?
                    .write_all(text.as_ref().as_bytes())?;
            }
            Flag::New => {
                let path = get_path_buf(slf)?;
                let file = fs::File::create(path);
                match file {
                    Ok(_) => {
                        slf.update_path = false;
                        let temp = get_path_buf(slf)?;
                        slf.flag = Flag::Auto;
                        fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(temp)?
                            .write_all(text.as_ref().as_bytes())?;
                    }
                    Err(err) => {
                        return Err(QPackError::IoError(err));
                    }
                };
            }
            Flag::Auto => {
                let path = get_path_buf(slf)?;
                let file: Result<fs::File, QPackError> = return_file(&path.to_str().unwrap());
                match file {
                    Ok(_) => {
                        slf.flag = Flag::Old;
                        auto_write(slf, text)?;
                    }
                    Err(err) => {
                        if let QPackError::IoError(err) = err {
                            match err.kind() {
                                _ => {
                                    path_create(slf, err.kind())?;
                                    auto_write(slf, text)?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    // #[async_recursion]
    // pub async fn async_auto_write<T: AsRef<str> + Send + Sync>(
    //     slf: &mut QFilePath,
    //     text: T,
    // ) -> Result<(), Box<dyn Error + Send + Sync>> {
    //     if slf.update_path {
    //         slf.correct_path = PathBuf::from(format!(
    //             "{}{}",
    //             slf.user_path.to_str().unwrap(),
    //             slf.file_name.to_str().unwrap()
    //         ));
    //     }
    //     match slf.flag {
    //         Flag::Old => {
    //             let temp = async_get_path_buf(slf).await?;
    //             slf.flag = Flag::Auto;
    //             async_fs::OpenOptions::new()
    //                 .append(true)
    //                 .open(temp)
    //                 .await?
    //                 .write_all(text.as_ref().as_bytes())
    //                 .await?;
    //         }
    //         Flag::New => {
    //             let path = async_get_path_buf(slf).await?;
    //             let file = async_fs::File::create(path).await;
    //             match file {
    //                 Ok(_) => {
    //                     slf.update_path = false;
    //                     let temp = async_get_path_buf(slf).await?;
    //                     slf.flag = Flag::Auto;
    //                     async_fs::OpenOptions::new()
    //                         .write(true)
    //                         .create(true)
    //                         .open(temp)
    //                         .await?
    //                         .write_all(text.as_ref().as_bytes())
    //                         .await?;
    //                 }
    //                 Err(err) => {
    //                     return Err(Box::new(err) as Box<dyn Error>);
    //                 }
    //             };
    //         }
    //         Flag::Auto => {
    //             let path = get_path_buf(slf)?;
    //             let file: Result<async_fs::File, Box<dyn Error>> =
    //                 async_return_file(&path.to_str().unwrap()).await;
    //             match file {
    //                 Ok(_) => {
    //                     slf.flag = Flag::Old;
    //                     async_auto_write(slf, text).await?;
    //                 }
    //                 Err(err) => {
    //                     if let Ok(err) = err.downcast::<std::io::Error>() {
    //                         match err.kind() {
    //                             _ => {
    //                                 path_create(slf, err.kind())?;
    //                                 async_auto_write(slf, text).await?;
    //                             }
    //                         }
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
}
