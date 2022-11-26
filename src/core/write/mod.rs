use crate::dpds_path::io::{self, Write};
use crate::dpds_path::{File, OpenOptions};

use super::get_file;

pub enum Flag {
    New,
    Auto,
    Old,
}

pub fn file_write(path: &str, text: &str, flag: Flag) -> Result<(), io::Error> {
    match flag {
        Flag::Auto => match get_file(path) {
            Ok(_) => return file_write(path, text, Flag::Old),
            Err(_) => return file_write(path, text, Flag::New),
        },
        Flag::New => match File::create(path) {
            Ok(_) => OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap()
                .write_all(text.as_bytes()),
            Err(err) => return Err(err.kind().into()),
        },
        Flag::Old => OpenOptions::new()
            .write(true)
            .append(true)
            .open(path)
            .unwrap()
            .write_all(text.as_bytes()),
    }
}
