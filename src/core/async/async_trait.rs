use super::super::{
    async_trait_crate::async_trait, AsyncArc, AsyncMutex, AsyncPack, AsyncPath, CodeStatus, Flag,
    QFilePath, QPathError,
};
use std::error::Error;
#[async_trait]
pub trait AsyncQPack {
    async fn async_new<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<AsyncArc<AsyncMutex<Self>>, Box<dyn Error>>;
    async fn async_correct_path(&mut self) -> Result<(), Box<dyn Error>>;
    async fn async_get_path_buf(&mut self) -> Result<&AsyncPath::PathBuf, Box<dyn Error>>;
    async fn async_get_path_str(&mut self) -> Result<&str, Box<dyn Error>>;
}
#[async_trait]
impl AsyncQPack for QFilePath {
    async fn async_new<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<AsyncArc<AsyncMutex<Self>>, Box<dyn Error>> {
        if path_file.as_ref().is_empty() {
            return Err(Box::new(QPathError::PathIsEmpty));
        }
        let path_file = AsyncPath::PathBuf::from(path_file.as_ref());
        if cfg!(unix) {
            if path_file.to_str().unwrap().contains("\\") {
                return Err(Box::new(QPathError::UnixPathIsIncorrect));
            }
        } else if cfg!(windows) {
            if path_file.to_str().unwrap().contains("/") {
                return Err(Box::new(QPathError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPathError::SystemNotDefined));
        }
        Ok(AsyncArc::new(AsyncMutex::new(QFilePath {
            Context: CodeStatus::AsyncCode(AsyncPack {
                request_items: Default::default(),
                only_file: None,
                user_path: AsyncPath::PathBuf::from(path_file.to_owned()),
                file_name: Default::default(),
                correct_path: Default::default(),
                flag: Flag::Auto,
                update_path: false,
            }),
        })))
    }
    async fn async_correct_path(&mut self) -> Result<(), Box<dyn Error>> {
        let mut counter = 0;
        if self
            .Context
            .get_async_pack_mut()
            .await
            .request_items
            .is_empty()
        {
            self.way_step_by_step().await;
        }
        let slf = self.get_context_mut().get_async_pack_mut().await;
        for user_i in 0..slf.request_items.len() {
            let mut possible_directories =
                QFilePath::async_directory_contents(slf.request_items[user_i].as_str()).await;
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
        if AsyncPath::Path::new(slf.request_items.last().unwrap())
            .exists()
            .await
        {
            slf.correct_path = AsyncPath::PathBuf::from(slf.request_items.last().unwrap());
        } else if cfg!(unix) {
            if AsyncPath::Path::new(&slf.request_items[counter])
                .exists()
                .await
                && counter != 0
            {
                slf.correct_path = AsyncPath::PathBuf::from(format!(
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
    async fn async_get_path_buf(&mut self) -> Result<&AsyncPath::PathBuf, Box<dyn Error>> {
        if cfg!(unix) {
            if self.Context.get_async_pack().await.user_path.exists().await {
                if !self
                    .Context
                    .get_async_pack()
                    .await
                    .correct_path
                    .to_str()
                    .unwrap()
                    .is_empty()
                {
                    return Ok(&self.Context.get_async_pack().await.correct_path);
                }
                return Ok(&self.Context.get_async_pack().await.user_path);
            }
            if !self.Context.get_async_pack().await.update_path
                && self
                    .Context
                    .get_async_pack()
                    .await
                    .correct_path
                    .to_str()
                    .unwrap()
                    .is_empty()
                && self
                    .Context
                    .get_async_pack()
                    .await
                    .user_path
                    .to_str()
                    .unwrap()
                    != self
                        .Context
                        .get_async_pack()
                        .await
                        .correct_path
                        .to_str()
                        .unwrap()
            {
                self.async_correct_path().await?;
                // self.Context.get_async_pack_mut().await.async_get_mutasync_correct_path();
            }
            if self
                .Context
                .get_async_pack()
                .await
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            {
                return Ok(&self.Context.get_async_pack().await.user_path);
            }
            return Ok(&self.Context.get_async_pack().await.correct_path);
        }
        if cfg!(windows) {
            if !self
                .Context
                .get_async_pack()
                .await
                .correct_path
                .exists()
                .await
            {
                self.async_correct_path().await?;
                // self.correct_path();
                if !self
                    .Context
                    .get_async_pack()
                    .await
                    .correct_path
                    .to_str()
                    .unwrap()
                    .is_empty()
                    && self.Context.get_async_pack().await.update_path
                {
                    let temp = self.Context.get_async_pack_mut().await.request_items.pop();
                    let last: String;
                    if self
                        .Context
                        .get_async_pack()
                        .await
                        .request_items
                        .last()
                        .unwrap()
                        != ".\\"
                        && !self
                            .Context
                            .get_async_pack()
                            .await
                            .request_items
                            .last()
                            .unwrap()
                            .contains(":\\")
                        && !self
                            .Context
                            .get_async_pack()
                            .await
                            .request_items
                            .last()
                            .unwrap()
                            .contains("..\\")
                    {
                        last = format!(
                            "{}\\{}",
                            self.Context
                                .get_async_pack_mut()
                                .await
                                .request_items
                                .pop()
                                .unwrap(),
                            self.Context
                                .get_async_pack()
                                .await
                                .file_name
                                .to_str()
                                .unwrap()
                        );
                    } else {
                        last = temp.unwrap();
                    }
                    self.Context.get_async_pack_mut().await.correct_path =
                        AsyncPath::PathBuf::from(last);
                    return Ok(&self.Context.get_async_pack().await.correct_path);
                }
            }
            if !self
                .Context
                .get_async_pack()
                .await
                .correct_path
                .to_str()
                .unwrap()
                .is_empty()
            {
                if self.Context.get_async_pack().await.update_path {
                    self.async_correct_path().await?;
                    // self.correct_path();
                }
                return Ok(&self.Context.get_async_pack().await.correct_path);
            }
            return Ok(&self.Context.get_async_pack().await.user_path);
        }
        return Err(Box::new(QPathError::SystemNotDefined));
    }
    async fn async_get_path_str(&mut self) -> Result<&str, Box<dyn Error>> {
        Ok(self.async_get_path_buf().await?.to_str().unwrap())
    }
}

// #[test]
// fn pack_test() {
//     task::block_on(async {
//         let file = QFilePath::async_new("src/Main.rs").await.unwrap();
//         file.lock().await.async_get_path_buf().await.unwrap();
//         let file_part1 = std::sync::Arc::clone(&file);
//         task::spawn(async move {
//             println!("{:#?}", file_part1.lock().await.Context);
//         });
//     });
// }
