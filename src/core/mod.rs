use self::{
    custom_errors::QPathError,
    traits::{PathPattern, PathPatternAsync},
};
use std::{collections::HashMap, env, error::Error};
use unicase::UniCase;
mod custom_errors;
mod default;
mod drop;
mod read;
mod traits;
mod write;
//=========================
#[derive(Debug)]
pub enum OptionCodeFile {
    SCFile(std::fs::File),
    ACFile(async_std::fs::File),
    UnknownStatusFile,
}
#[derive(Debug)]
pub enum OptionCodePathBuf {
    SCPathBuf(std::path::PathBuf),
    ACPathBuf(async_std::path::PathBuf),
    UnknownStatusPathBuf,
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
pub struct QFilePath<'a> {
    request_items: HashMap<UniCase<&'a String>, String>,
    only_file: OptionCodeFile,
    user_path: OptionCodePathBuf,
    file_name: OptionCodePathBuf,
    correct_path: OptionCodePathBuf,
    flag: Flag,
    update_path: bool,
    pattern: QPatternPath,
}
//SC
impl<'a> QFilePath<'a> {
    pub fn new<P: AsRef<str>>(path_file: P) -> Result<Self, Box<dyn Error>> {
        let user_path = QFilePath::init_user_path(path_file)?;
        let file_name = if let OptionCodePathBuf::SCPathBuf(file_path) = &user_path {
            OptionCodePathBuf::SCPathBuf(std::path::PathBuf::from(file_path.file_name().unwrap()))
        } else {
            OptionCodePathBuf::UnknownStatusPathBuf
        };
        Ok(Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path,
            file_name,
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
            pattern: Default::default(),
        })
    }
    pub fn change_path<P: AsRef<str>>(&mut self, path_file: P) -> Result<(), Box<dyn Error>> {
        self.user_path = QFilePath::init_user_path(path_file)?;
        Ok(())
    }
}
//AC
impl<'a> QFilePath<'a> {
    pub async fn async_new<P: AsRef<str> + std::marker::Send>(
        path_file: P,
    ) -> Result<QFilePath<'a>, Box<dyn Error>> {
        let user_path = QFilePath::async_init_user_path(path_file).await?;
        let file_name = if let OptionCodePathBuf::ACPathBuf(file_path) = &user_path {
            OptionCodePathBuf::ACPathBuf(async_std::path::PathBuf::from(
                file_path.file_name().unwrap(),
            ))
        } else {
            OptionCodePathBuf::UnknownStatusPathBuf
        };
        Ok(Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path,
            file_name,
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
            pattern: Default::default(),
        })
    }
    pub async fn async_change_path<P: AsRef<str> + std::marker::Send>(
        &mut self,
        path_file: P,
    ) -> Result<(), Box<dyn Error>> {
        self.user_path = QFilePath::async_init_user_path(path_file).await?;
        Ok(())
    }
}
