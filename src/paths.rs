use super::{PathBuf, QFilePath, QPackError};
use crate::init::correct_path::*;
pub mod get_path {
    use super::*;
    pub fn get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, QPackError> {
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
                correct_path(slf);
            }
            if slf.correct_path.to_str().unwrap().is_empty() {
                return Ok(PathBuf::from(slf.user_path.to_path_buf()));
            }
            return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
        }
        if cfg!(windows) {
            if !slf.correct_path.exists() {
                correct_path(slf);
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
                    correct_path(slf);
                }
                return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
            }
            return Ok(PathBuf::from(slf.user_path.to_path_buf()));
        }
        return Err(QPackError::SystemNotDefined);
    }
    pub fn get_path_string(slf: &mut QFilePath) -> Result<String, QPackError> {
        Ok(get_path_buf(slf)?.display().to_string())
    }
    pub async fn async_get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, QPackError> {
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
                async_correct_path(slf).await;
            }
            if slf.correct_path.to_str().unwrap().is_empty() {
                return Ok(PathBuf::from(slf.user_path.to_path_buf()));
            }
            return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
        }
        if cfg!(windows) {
            if !slf.correct_path.exists() {
                async_correct_path(slf).await;
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
                    async_correct_path(slf).await;
                }
                return Ok(PathBuf::from(slf.correct_path.to_path_buf()));
            }
            return Ok(PathBuf::from(slf.user_path.to_path_buf()));
        }
        return Err(QPackError::SystemNotDefined);
        // .map_err(|err| {
        //     let boxed: Box<dyn Error + Send + Sync> =
        //         Box::new(*err.downcast::<QPackError>().unwrap());
        //     boxed
        // })
    }
    pub async fn async_get_path_string(slf: &mut QFilePath) -> Result<String, QPackError> {
        Ok(async_get_path_buf(slf).await?.display().to_string())
    }
}
