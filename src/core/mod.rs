use std::{collections::HashMap, env, error::Error, path::PathBuf};
use unicase::UniCase;

use self::custom_errors::QPathError;
mod custom_errors;
mod default;
mod drop;
mod read;
mod write;
//=========================
static mut OS_QFILE: &str = env::consts::OS;
static mut OS_status: OS<&str> = OS::KnownOS;
#[derive(Debug)]
enum OptionCodeFile {
    SCFile(std::fs::File),
    ACFile(async_std::fs::File),
    UnknownStatusFile,
}

#[derive(Debug)]
pub enum Flag {
    New,
    Auto,
    Old,
}

#[derive(Debug)]
enum OS<T: AsRef<str>> {
    UserSelectedOS(T),
    KnownOS,
}
#[derive(Debug)]
pub struct QFilePath<'a> {
    request_items: HashMap<UniCase<&'a String>, String>,
    only_file: OptionCodeFile,
    user_path: PathBuf,
    file_name: PathBuf,
    correct_path: PathBuf,
    flag: Flag,
    update_path: bool,
}
impl<'a> QFilePath<'a> {
    pub fn set_os(name_os: &'static str) {
        unsafe {
            OS_QFILE = name_os;
        }
    }
    pub fn new<T: AsRef<str>>(path: T) -> Result<Self, Box<dyn Error>> {
        Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: init_path(path)?,
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
        }
    }
    pub fn change_path(&mut self) {}
}

fn init_path<'a, T: AsRef<str>>(path: T) -> Result<T, Box<dyn Error>> {
    match unsafe { OS_status } {
        OS::UserSelectedOS(_) | OS::KnownOS => {
            if path.as_ref().is_empty() {
                return Err(Box::new(QPathError::PathIsEmpty));
            }
            if cfg!(windows) {
                return Ok();
            }
            if cfg!(unix) {
                return Ok();
            };
        }
    }
    Err(Box::new(QPathError::SystemNotDefined))
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
