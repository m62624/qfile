use super::{Error, PathBuf, QFilePath, QPackError};
use std::path::Path;
pub fn correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error>> {
    let mut counter = 0;
    if slf.context.get_sync_pack().request_items.is_empty() {
        slf.way_step_by_step();
    }
    for user_i in 0..slf.context.get_sync_pack().request_items.len() {
        let mut possible_directories = QFilePath::directory_contents(
            slf.context.get_sync_pack().request_items[user_i].as_str(),
        );
        for pos_j in 0..possible_directories.len() {
            if slf
                .context
                .get_sync_pack()
                .request_items
                .get(user_i + 1)
                .unwrap_or(
                    &slf.context
                        .get_sync_pack()
                        .request_items
                        .get(user_i)
                        .unwrap()
                        .to_lowercase(),
                )
                .to_lowercase()
                == possible_directories[pos_j].to_lowercase()
            {
                slf.context.get_sync_pack_mut().request_items[user_i + 1] =
                    possible_directories.remove(pos_j);
                counter += 1;
                break;
            }
        }
    }
    if Path::new(slf.context.get_sync_pack().request_items.last().unwrap()).exists() {
        slf.context.get_sync_pack_mut().correct_path =
            PathBuf::from(slf.context.get_sync_pack().request_items.last().unwrap());
    } else if cfg!(unix) {
        if Path::new(&slf.context.get_sync_pack().request_items[counter]).exists() && counter != 0 {
            slf.context.get_sync_pack_mut().correct_path = PathBuf::from(format!(
                "{}{}",
                slf.context.get_sync_pack().request_items[counter],
                slf.context
                    .get_sync_pack()
                    .request_items
                    .last()
                    .unwrap()
                    .split_at(slf.context.get_sync_pack().request_items[counter].len())
                    .1
            ));
        }
    }
    Ok(())
}
pub fn get_path_buf(slf: &mut QFilePath) -> Result<PathBuf, Box<dyn Error>> {
    if cfg!(unix) {
        if slf.context.get_sync_pack().user_path.exists() {
            if !slf
                .context
                .get_sync_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            {
                return Ok(PathBuf::from(
                    slf.context.get_sync_pack().correct_path.to_path_buf(),
                ));
            }
            return Ok(PathBuf::from(
                slf.context.get_sync_pack_mut().user_path.to_path_buf(),
            ));
        }
        if !slf.context.get_sync_pack().update_path
            && slf
                .context
                .get_sync_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            && slf.context.get_sync_pack().user_path.to_str().unwrap()
                != slf.context.get_sync_pack().correct_path.to_str().unwrap()
        {
            correct_path(slf)?;
        }
        if slf
            .context
            .get_sync_pack()
            .correct_path
            .to_str()
            .unwrap()
            .is_empty()
        {
            return Ok(PathBuf::from(
                slf.context.get_sync_pack().user_path.to_path_buf(),
            ));
        }
        return Ok(PathBuf::from(
            slf.context.get_sync_pack().correct_path.to_path_buf(),
        ));
    }
    if cfg!(windows) {
        if !slf.context.get_sync_pack().correct_path.exists() {
            correct_path(slf)?;
            if !slf
                .context
                .get_sync_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
                && slf.context.get_sync_pack().update_path
            {
                let temp = slf.context.get_sync_pack_mut().request_items.pop();
                let last: String;
                if slf.context.get_sync_pack().request_items.last().unwrap() != ".\\"
                    && !slf
                        .context
                        .get_sync_pack()
                        .request_items
                        .last()
                        .unwrap()
                        .contains(":\\")
                    && !slf
                        .context
                        .get_sync_pack()
                        .request_items
                        .last()
                        .unwrap()
                        .contains("..\\")
                {
                    last = format!(
                        "{}\\{}",
                        slf.context.get_sync_pack_mut().request_items.pop().unwrap(),
                        slf.context.get_sync_pack().file_name.to_str().unwrap()
                    );
                } else {
                    last = temp.unwrap();
                }
                slf.context.get_sync_pack_mut().correct_path = PathBuf::from(last);
                return Ok(PathBuf::from(
                    slf.context.get_sync_pack().correct_path.to_path_buf(),
                ));
            }
        }
        if !slf
            .context
            .get_sync_pack()
            .correct_path
            .to_str()
            .unwrap()
            .is_empty()
        {
            if slf.context.get_sync_pack().update_path {
                correct_path(slf)?;
            }
            return Ok(PathBuf::from(
                slf.context.get_sync_pack().correct_path.to_path_buf(),
            ));
        }
        return Ok(PathBuf::from(
            slf.context.get_sync_pack().user_path.to_path_buf(),
        ));
    }
    return Err(Box::new(QPackError::SystemNotDefined));
}
