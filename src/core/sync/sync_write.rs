use super::{Error, QFilePath};
use crate::core::{sync::sync_trait::QFileSync, Flag};
use std::io::Write;
use std::{fs, path::PathBuf};
pub fn auto_write<T: AsRef<str>>(slf: &mut QFilePath, text: T) -> Result<(), Box<dyn Error>> {
    //=======================================================
    let sl = slf.context.get_sync_pack();
    //=======================================================
    if sl.update_path {
        if cfg!(unix) {
            slf.context.get_sync_pack_mut().correct_path = PathBuf::from(format!(
                "{}{}",
                sl.user_path.to_str().unwrap(),
                sl.file_name.to_str().unwrap()
            ))
        }
        //=======================================================
        let sl = slf.context.get_sync_pack();
        //=======================================================
        if cfg!(windows) {
            slf.context.get_sync_pack_mut().correct_path = PathBuf::from(format!(
                "{}{}",
                sl.user_path.to_str().unwrap(),
                sl.file_name.to_str().unwrap()
            ))
        }
    }
    //=======================================================
    let sl = slf.context.get_sync_pack();
    //=======================================================
    match sl.flag {
        crate::core::Flag::Old => {
            let temp = QFilePath::get_path_buf(slf)?;
            slf.context.get_sync_pack_mut().flag = Flag::Auto;
            fs::OpenOptions::new()
                .append(true)
                .open(temp)?
                .write_all(text.as_ref().as_bytes())?;
        }
        Flag::New => {
            let path = QFilePath::get_path_buf(slf)?;
            let file = fs::File::create(path);
            match file {
                Ok(_) => {
                    let temp = QFilePath::get_path_buf(slf)?;
                    slf.context.get_sync_pack_mut().update_path = false;
                    slf.context.get_sync_pack_mut().flag = Flag::Auto;
                    fs::OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(temp)?
                        .write_all(text.as_ref().as_bytes())?;
                }
                Err(err) => {
                    return Err(Box::new(err) as Box<dyn Error>);
                }
            };
        }
        Flag::Auto => {
            let path = QFilePath::get_path_buf(slf)?;
            let file: Result<fs::File, Box<dyn Error>> =
                QFilePath::return_file(&path.to_str().unwrap());
            match file {
                Ok(_) => {
                    slf.context.get_sync_pack_mut().flag = Flag::Old;
                    auto_write(slf, text)?;
                }
                Err(err) => {
                    if let Ok(err) = err.downcast::<std::io::Error>() {
                        match err.kind() {
                            _ => {
                                let dir = QFilePath::dir_create(slf, err.kind());
                                dir.unwrap();
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
pub fn write_only_new<T: AsRef<str>>(slf: &mut QFilePath, text: T) -> Result<(), Box<dyn Error>> {
    slf.context.get_sync_pack_mut().flag = Flag::New;
    if let Err(err) = auto_write(slf, &text) {
        if let Ok(err) = err.downcast::<std::io::Error>() {
            match err.kind() {
                _ => {
                    QFilePath::dir_create(slf, err.kind())?;
                    auto_write(slf, &text)?;
                }
            }
        }
    }
    Ok(())
}
