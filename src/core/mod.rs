mod read;
mod write;
use std::env;
pub struct QPack<'a> {
    possible_directories: Vec<String>,
    user_path: &'a str,
    correct_path: &'a str,
    os: &'a str,
}

//======================================================
impl<'a> QPack<'a> {
    pub fn add_path(path: &'a str) -> Self {
        QPack {
            possible_directories: Default::default(),
            user_path: path,
            correct_path: Default::default(),
            os: env::consts::OS,
        }
    }
}
