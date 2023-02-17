use super::{PathBuf, QFilePath, QPackError};
use crate::init::correct_path::*;
use std::error::Error;
pub fn get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, Box<dyn Error>> {
    if cfg!(unix) {
        if slf.user_path.exists() {
            if !slf.correct_path.to_str().unwrap().is_empty() {
                return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
            }
            return Ok(PathBuf::from(slf.user_path.to_path_buf()));
        }
        if !slf.update_path
            && slf.correct_path.to_str().unwrap().is_empty()
            && slf.user_path.to_str().unwrap() != slf.correct_path.to_str().unwrap()
        {
            correct_path(slf)?;
        }
        if slf.correct_path.to_str().unwrap().is_empty() {
            return Ok(PathBuf::from(slf.user_path.to_path_buf()));
        }
        return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
    }
    if cfg!(windows) {
        if !slf.correct_path.exists() {
            correct_path(slf)?;
            if !slf.correct_path.to_str().unwrap().is_empty() && slf.update_path {
                let temp = slf.request_items.pop();
                let last: String;
                if slf.request_items.last().unwrap() != ".\\"
                    && !slf.request_items.last().unwrap().contains(":\\")
                    && !slf.request_items.last().unwrap().contains("..\\")
                {
                    last = format!(
                        "{}\\{}",
                        slf.request_items.pop().unwrap(),
                        slf.file_name.to_str().unwrap()
                    );
                } else {
                    last = temp.unwrap();
                }
                slf.correct_path = PathBuf::from(last);
                return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
            }
        }
        if !slf.correct_path.to_str().unwrap().is_empty() {
            if slf.update_path {
                correct_path(slf)?;
            }
            return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
        }
        return Ok(PathBuf::from(slf.user_path.to_path_buf()));
    }
    return Err(Box::new(QPackError::SystemNotDefined));
}
pub fn get_path_string(slf: &mut QFilePath) -> Result<String, Box<dyn Error>> {
    Ok(get_path_buf(slf)?.display().to_string())
}
