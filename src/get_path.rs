use super::PathBuf;
use super::{Error, QFilePath, QPackError};
use std::path::Path;
pub fn correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error>> {
    let mut counter = 0;
    if slf.request_items.is_empty() {
        slf.way_step_by_step();
    }
    for user_i in 0..slf.request_items.len() {
        let mut possible_directories =
            QFilePath::directory_contents(slf.request_items[user_i].as_str());
        for pos_j in 0..possible_directories.len() {
            if slf
                .request_items
                .get(user_i + 1)
                .unwrap_or(&slf.request_items.get(user_i).unwrap().to_lowercase())
                .to_lowercase()
                == possible_directories[pos_j].to_lowercase()
            {
                slf.request_items[user_i + 1] = possible_directories.remove(pos_j);
                counter += 1;
                break;
            }
        }
    }
    if Path::new(slf.request_items.last().unwrap()).exists() {
        slf.correct_path = PathBuf::from(slf.request_items.last().unwrap());
    } else if cfg!(unix) {
        if Path::new(&slf.request_items[counter]).exists() && counter != 0 {
            slf.correct_path = PathBuf::from(format!(
                "{}{}",
                slf.request_items[counter],
                slf.request_items
                    .last()
                    .unwrap()
                    .split_at(slf.request_items[counter].len())
                    .1
            ));
        }
    }
    Ok(())
}
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
