use super::super::{get_path_buf, Error, Flag, QFilePath};
use std::io::Write;
use std::{fs, path::PathBuf};
pub fn auto_write<T: AsRef<str>>(slf: &mut QFilePath, text: T) -> Result<(), Box<dyn Error>> {
    if slf.update_path {
        #[cfg(unix)]
        {
            slf.correct_path = PathBuf::from(format!(
                "{}{}",
                slf.user_path.to_str().unwrap(),
                slf.file_name.to_str().unwrap()
            ))
        }
        #[cfg(windows)]
        {
            slf.correct_path = PathBuf::from(format!(
                "{}{}",
                slf.user_path.to_str().unwrap(),
                slf.file_name.to_str().unwrap()
            ))
        }
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
                    slf.flag = Flag::Old;
                    auto_write(slf, text)?;
                }
                Err(err) => {
                    if let Ok(err) = err.downcast::<std::io::Error>() {
                        match err.kind() {
                            _ => {
                                QFilePath::path_create(slf, err.kind())?;
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
    slf.flag = Flag::New;
    if let Err(err) = auto_write(slf, &text) {
        if let Ok(err) = err.downcast::<std::io::Error>() {
            match err.kind() {
                _ => {
                    QFilePath::path_create(slf, err.kind())?;
                    auto_write(slf, &text)?;
                }
            }
        }
    }
    Ok(())
}
