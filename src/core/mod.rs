use self::{
    custom_errors::QPathError,
    traits::{PathPattern, PathPatternAsync},
};
use async_std::sync::Arc as ACArc;
use async_std::sync::Mutex as ACMutex;
use std::{collections::HashMap, env, error::Error};
use unicase::UniCase;
mod custom_errors;
use lazy_static::lazy_static;
use regex::Regex;
mod default;
mod drop;
mod read;
mod traits;
mod write;
//=========================
#[derive(Debug)]
pub enum OptionCodeFile {
    SCFile(std::fs::File),
    // ACFile(ACMutex<async_std::fs::File>),
    UnknownStatusFile,
}
#[derive(Debug)]
pub enum OptionCodePathBuf {
    SCPathBuf(std::path::PathBuf),
    ACPathBuf(ACArc<ACMutex<async_std::path::PathBuf>>),
    UnknownStatusPathBuf,
}
#[derive(Debug)]
pub enum OptionCodeRequestItems {
    SCRequestItems(Vec<String>),
    ACRequestItems(ACArc<ACMutex<Vec<String>>>),
    UnknownStatusRequestItems,
}

#[derive(Debug)]
pub enum Flag {
    New,
    Auto,
    Old,
}

#[derive(Debug)]
pub enum QPatternPath {
    NewPattern,
    DefaultPattern,
}
#[derive(Debug)]
pub struct QFilePath {
    request_items: OptionCodeRequestItems,
    only_file: OptionCodeFile,
    user_path: OptionCodePathBuf,
    file_name: OptionCodePathBuf,
    correct_path: OptionCodePathBuf,
    flag: Flag,
    update_path: bool,
    pattern: QPatternPath,
}
impl QFilePath {
    async fn async_new<T: AsRef<str>>(path_file: T) -> Result<QFilePath, Box<dyn Error>> {
        Ok(Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: QFilePath::async_init_user_path(path_file.as_ref()).await?,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
            pattern: Default::default(),
        })
    }
}
impl QFilePath {
    async fn first_slash(&mut self, capsule: ACArc<ACMutex<async_std::path::PathBuf>>) {
        let temp = capsule.as_ref().lock().await.display().to_string();
        if cfg!(target_family = "unix") {
            lazy_static! {
                static ref SL: Regex = Regex::new(r"^/|^\.\./|^\./").unwrap();
            }
            if !SL.is_match(&temp) {
                self.user_path = OptionCodePathBuf::ACPathBuf(ACArc::new(ACMutex::new(
                    async_std::path::PathBuf::from(format!("./{}", temp)),
                )));
            }
        }
        if cfg!(target_family = "windows") {
            lazy_static! {
                static ref SL: Regex = Regex::new(r"^.:\\|^\.\.\\|^\.\\").unwrap();
            }
            if !SL.is_match(&temp) {
                self.user_path = OptionCodePathBuf::ACPathBuf(ACArc::new(ACMutex::new(
                    async_std::path::PathBuf::from(format!(".\\{}", temp)),
                )));
            }
        }
    }
}
