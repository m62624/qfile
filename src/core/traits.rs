use super::{ACArc, ACMutex, OptionCodePathBuf, QFilePath, QPathError};
use crate::core::OptionCodeRequestItems;
use async_trait::async_trait;
use std::error::Error;
pub trait PathPattern {
    fn init_user_path<T: AsRef<str>>(path_file: T) -> Result<OptionCodePathBuf, Box<dyn Error>>;
    // fn correct_path(&mut self);
}
#[async_trait]
pub trait PathPatternAsync {
    async fn async_init_user_path<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<OptionCodePathBuf, Box<dyn Error>>;
    async fn way_step_by_step(&mut self);
    async fn async_correct_path(&mut self) {
        self.way_step_by_step().await;
    }
}
#[async_trait]
impl PathPatternAsync for QFilePath {
    async fn async_init_user_path<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<OptionCodePathBuf, Box<dyn Error>> {
        if path_file.as_ref().is_empty() {
            return Err(Box::new(QPathError::PathIsEmpty));
        }
        let path_file = async_std::path::PathBuf::from(path_file.as_ref());
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
        return Ok(OptionCodePathBuf::ACPathBuf(ACArc::new(ACMutex::new(
            path_file,
        ))));
    }
    async fn way_step_by_step(&mut self) {
        let capsule = if let OptionCodePathBuf::ACPathBuf(value) = &self.user_path {
            value.clone()
        } else {
            panic!("Status OptionCodePathBuf: {:#?}", self.user_path);
        };
        let capsule_clone = ACArc::clone(&capsule);
        self.first_slash(capsule_clone).await;
        let mut temp: Vec<String> = capsule
            .as_ref()
            .lock()
            .await
            .ancestors()
            .map(|element| element.display().to_string())
            .collect();
        if temp.last().unwrap().eq("") {
            temp.pop();
            if let Some(value) = temp.last_mut() {
                if cfg!(unix) {
                    if value.eq(&mut ".") {
                        *value = String::from("./")
                    }
                    if value.eq(&mut "..") {
                        *value = String::from("../")
                    }
                }
                if cfg!(windows) {
                    if value.eq(&mut ".") {
                        *value = String::from(".\\")
                    }
                    if value.eq(&mut "..") {
                        *value = String::from("..\\")
                    }
                }
            }
        }
        temp.reverse();
        self.request_items = OptionCodeRequestItems::ACRequestItems(ACArc::new(ACMutex::new(temp)));
    }
    async fn async_correct_path(&mut self) {
        self.way_step_by_step().await;
    }
}
