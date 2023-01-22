use std::{collections::HashMap, env, error::Error};
use unicase::UniCase;

use self::{
    custom_errors::QPathError,
    traits::{PathPattern, PathPatternAsync},
};
mod custom_errors;
mod default;
mod drop;
mod traits;
//=========================
static mut OS_QFILE: &str = env::consts::OS;
#[derive(Debug)]
enum OptionCodeFile {
    SCFile(std::fs::File),
    ACFile(async_std::fs::File),
    UnknownStatusFile,
}
#[derive(Debug)]
enum OptionCodePathBuf {
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
enum OS<T: AsRef<str>> {
    NewPattern(T),
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
}
impl<'a> QFilePath<'a> {
    pub fn set_os(name_os: &'static str) {
        unsafe {
            OS_QFILE = name_os;
        }
    }
    pub fn new<T: AsRef<str>>(path_file: T) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: QFilePath::init_user_path(path_file)?,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
        })
    }
    pub async fn async_new<T: AsRef<str> + std::marker::Send>(
        path_file: T,
    ) -> Result<QFilePath<'a>, Box<dyn Error>> {
        Ok(Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: QFilePath::async_init_user_path(path_file).await?,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
        })
    }
    pub fn change_path(&mut self) {}
}

#[cfg(test)]
mod test_polygon {
    use crate::{
        core::{env, OS_QFILE},
        QFilePath,
    };
    use rand::Rng;
    use std::iter;
    fn generate(len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-";
        let mut rng = rand::thread_rng();
        let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        iter::repeat_with(one_char).take(len).collect()
    }
    #[test]
    fn os_status() {
        QFilePath::set_os(Box::leak(generate(25).into_boxed_str()));
        assert_ne!(unsafe { OS_QFILE }, env::consts::OS);
    }
}
