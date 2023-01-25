use super::{Error, PathBuf, QFilePath, QPackError};
use std::path::Path;
pub fn correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error + Send + Sync>> {
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
