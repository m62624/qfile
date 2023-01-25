use super::{AsyncPath, Error, QFilePath, QPackError};

pub async fn async_correct_path(slf: &mut QFilePath) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut counter = 0;
    if slf.context.get_async_pack().request_items.is_empty() {
        slf.async_way_step_by_step().await;
    }
    for user_i in 0..slf.context.get_async_pack().request_items.len() {
        let mut possible_directories = QFilePath::async_directory_contents(
            slf.context.get_async_pack().request_items[user_i].as_str(),
        )
        .await;
        for pos_j in 0..possible_directories.len() {
            if slf
                .context
                .get_async_pack()
                .request_items
                .get(user_i + 1)
                .unwrap_or(
                    &slf.context
                        .get_async_pack()
                        .request_items
                        .get(user_i)
                        .unwrap()
                        .to_lowercase(),
                )
                .to_lowercase()
                == possible_directories[pos_j].to_lowercase()
            {
                slf.context.get_async_pack_mut().request_items[user_i + 1] =
                    possible_directories.remove(pos_j);
                counter += 1;
                break;
            }
        }
    }
    if AsyncPath::Path::new(slf.context.get_async_pack().request_items.last().unwrap())
        .exists()
        .await
    {
        slf.context.get_async_pack_mut().correct_path =
            AsyncPath::PathBuf::from(slf.context.get_async_pack().request_items.last().unwrap());
    } else if cfg!(unix) {
        if AsyncPath::Path::new(&slf.context.get_async_pack().request_items[counter])
            .exists()
            .await
            && counter != 0
        {
            slf.context.get_async_pack_mut().correct_path = AsyncPath::PathBuf::from(format!(
                "{}{}",
                slf.context.get_async_pack().request_items[counter],
                slf.context
                    .get_async_pack()
                    .request_items
                    .last()
                    .unwrap()
                    .split_at(slf.context.get_async_pack().request_items[counter].len())
                    .1
            ));
        }
    }
    Ok(())
}

pub async fn async_get_path_buf(
    slf: &mut QFilePath,
) -> Result<AsyncPath::PathBuf, Box<dyn Error + Send + Sync>> {
    if cfg!(unix) {
        if slf.context.get_async_pack().user_path.exists().await {
            if !slf
                .context
                .get_async_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            {
                return Ok(AsyncPath::PathBuf::from(
                    slf.context.get_async_pack().correct_path.to_path_buf(),
                ));
            }
            return Ok(AsyncPath::PathBuf::from(
                slf.context.get_async_pack_mut().user_path.to_path_buf(),
            ));
        }
        if !slf.context.get_async_pack().update_path
            && slf
                .context
                .get_async_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            && slf.context.get_async_pack().user_path.to_str().unwrap()
                != slf.context.get_async_pack().correct_path.to_str().unwrap()
        {
            async_correct_path(slf).await?;
        }
        if slf
            .context
            .get_async_pack()
            .correct_path
            .to_str()
            .unwrap()
            .is_empty()
        {
            return Ok(AsyncPath::PathBuf::from(
                slf.context.get_async_pack().user_path.to_path_buf(),
            ));
        }
        return Ok(AsyncPath::PathBuf::from(
            slf.context.get_async_pack().correct_path.to_path_buf(),
        ));
    }
    if cfg!(windows) {
        if !slf.context.get_async_pack().correct_path.exists().await {
            async_correct_path(slf).await?;
            if !slf
                .context
                .get_async_pack()
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
                && slf.context.get_async_pack().update_path
            {
                let temp = slf.context.get_async_pack_mut().request_items.pop();
                let last: String;
                if slf.context.get_async_pack().request_items.last().unwrap() != ".\\"
                    && !slf
                        .context
                        .get_async_pack()
                        .request_items
                        .last()
                        .unwrap()
                        .contains(":\\")
                    && !slf
                        .context
                        .get_async_pack()
                        .request_items
                        .last()
                        .unwrap()
                        .contains("..\\")
                {
                    last = format!(
                        "{}\\{}",
                        slf.context
                            .get_async_pack_mut()
                            .request_items
                            .pop()
                            .unwrap(),
                        slf.context.get_async_pack().file_name.to_str().unwrap()
                    );
                } else {
                    last = temp.unwrap();
                }
                slf.context.get_async_pack_mut().correct_path = AsyncPath::PathBuf::from(last);
                return Ok(AsyncPath::PathBuf::from(
                    slf.context.get_async_pack().correct_path.to_path_buf(),
                ));
            }
        }
        if !slf
            .context
            .get_async_pack()
            .correct_path
            .to_str()
            .unwrap()
            .is_empty()
        {
            if slf.context.get_async_pack().update_path {
                async_correct_path(slf).await?;
            }
            return Ok(AsyncPath::PathBuf::from(
                slf.context.get_async_pack().correct_path.to_path_buf(),
            ));
        }
        return Ok(AsyncPath::PathBuf::from(
            slf.context.get_async_pack().user_path.to_path_buf(),
        ));
    }
    return Err(Box::new(QPackError::SystemNotDefined));
}
