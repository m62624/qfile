use super::super::{
    async_trait_crate::async_trait, AsyncArc, AsyncMutex, AsyncPack, AsyncPath, CodeStatus, Flag,
    QFilePath, QPackError,
};
use async_std::task;
use std::error::Error;
#[async_trait]
pub trait AsyncQPackTrait {
    async fn async_new<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<AsyncArc<AsyncMutex<Self>>, Box<dyn Error>>;
    async fn async_correct_path(self: &mut Self) -> Result<(), Box<dyn Error>>;
    async fn async_get_path_buf(self: &mut Self) -> Result<AsyncPath::PathBuf, Box<dyn Error>>;
}
#[async_trait]
impl AsyncQPackTrait for QFilePath {
    async fn async_new<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<AsyncArc<AsyncMutex<Self>>, Box<dyn Error>> {
        if path_file.as_ref().is_empty() {
            return Err(Box::new(QPackError::PathIsEmpty));
        }
        let path_file = AsyncPath::PathBuf::from(path_file.as_ref());
        if cfg!(unix) {
            if path_file.to_str().unwrap().contains("\\") {
                return Err(Box::new(QPackError::UnixPathIsIncorrect));
            }
        } else if cfg!(windows) {
            if path_file.to_str().unwrap().contains("/") {
                return Err(Box::new(QPackError::WindowsPathIsIncorrect));
            }
        } else {
            return Err(Box::new(QPackError::SystemNotDefined));
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
    async fn async_get_path_buf(self: &mut Self) -> Result<AsyncPath::PathBuf, Box<dyn Error>> {
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
                    return Ok(AsyncPath::PathBuf::from(
                        self.Context
                            .get_async_pack()
                            .await
                            .correct_path
                            .to_path_buf(),
                    ));
                }
                return Ok(AsyncPath::PathBuf::from(
                    self.Context.get_async_pack().await.user_path.to_path_buf(),
                ));
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
                QFilePath::async_correct_path(self).await?;
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
                return Ok(AsyncPath::PathBuf::from(
                    self.Context.get_async_pack().await.user_path.to_path_buf(),
                ));
            }
            return Ok(AsyncPath::PathBuf::from(
                self.Context
                    .get_async_pack()
                    .await
                    .correct_path
                    .to_path_buf(),
            ));
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
                QFilePath::async_correct_path(self).await?;
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
                    return Ok(AsyncPath::PathBuf::from(
                        self.Context
                            .get_async_pack()
                            .await
                            .correct_path
                            .to_path_buf(),
                    ));
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
                    QFilePath::async_correct_path(self).await?;
                }
                return Ok(AsyncPath::PathBuf::from(
                    self.Context
                        .get_async_pack()
                        .await
                        .correct_path
                        .to_path_buf(),
                ));
            }
            return Ok(AsyncPath::PathBuf::from(
                self.Context.get_async_pack().await.user_path.to_path_buf(),
            ));
        }
        return Err(Box::new(QPackError::SystemNotDefined));
    }

    async fn async_correct_path(self: &mut Self) -> Result<(), Box<dyn Error>> {
        let mut counter = 0;
        if self.Context.get_async_pack().await.request_items.is_empty() {
            self.way_step_by_step().await;
        }
        for user_i in 0..self.Context.get_async_pack().await.request_items.len() {
            let mut possible_directories = QFilePath::async_directory_contents(
                self.Context.get_async_pack().await.request_items[user_i].as_str(),
            )
            .await;
            for pos_j in 0..possible_directories.len() {
                if self
                    .Context
                    .get_async_pack()
                    .await
                    .request_items
                    .get(user_i + 1)
                    .unwrap_or(
                        &self
                            .Context
                            .get_async_pack()
                            .await
                            .request_items
                            .get(user_i)
                            .unwrap()
                            .to_lowercase(),
                    )
                    .to_lowercase()
                    == possible_directories[pos_j].to_lowercase()
                {
                    self.Context.get_async_pack_mut().await.request_items[user_i + 1] =
                        possible_directories.remove(pos_j);
                    counter += 1;
                    break;
                }
            }
        }
        if AsyncPath::Path::new(
            self.Context
                .get_async_pack()
                .await
                .request_items
                .last()
                .unwrap(),
        )
        .exists()
        .await
        {
            self.Context.get_async_pack_mut().await.correct_path = AsyncPath::PathBuf::from(
                self.Context
                    .get_async_pack()
                    .await
                    .request_items
                    .last()
                    .unwrap(),
            );
        } else if cfg!(unix) {
            if AsyncPath::Path::new(&self.Context.get_async_pack().await.request_items[counter])
                .exists()
                .await
                && counter != 0
            {
                self.Context.get_async_pack_mut().await.correct_path =
                    AsyncPath::PathBuf::from(format!(
                        "{}{}",
                        self.Context.get_async_pack().await.request_items[counter],
                        self.Context
                            .get_async_pack()
                            .await
                            .request_items
                            .last()
                            .unwrap()
                            .split_at(
                                self.Context.get_async_pack().await.request_items[counter].len()
                            )
                            .1
                    ));
            }
        }
        Ok(())
    }
}
