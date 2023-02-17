use super::{Flag, PathBuf, QFilePath, QPackError};
use crate::paths::get_path::*;
use async_fs;
use std::error::Error;
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
            let temp = get_path_buf(slf)?;
            slf.context.get_sync_pack_mut().flag = Flag::Auto;
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
                    slf.context.get_sync_pack_mut().update_path = false;
                    let temp = get_path_buf(slf)?;
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
            let path = get_path_buf(slf)?;
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
