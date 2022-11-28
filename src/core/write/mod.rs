pub mod files;
use self::files::{collect_folder, correct_path};
use super::get_file;
use crate::dpds_path::io::{self, ErrorKind, Write};
use crate::dpds_path::{DirBuilder, File, OpenOptions};

pub enum Flag {
    New,
    Auto,
    Old,
}
pub fn file_write(path: &str, text: &str, flag: Flag) -> Result<(), io::Error> {
    match flag {
        Flag::Auto => match get_file(path) {
            Ok(_) => return file_write(path, text, Flag::Old),
            Err(_) => match correct_path(path) {
                Ok(new_path) => file_write(&new_path, text, Flag::Old),
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => Ok({
                        let mut temp = collect_folder(path);
                        let name = temp.pop().unwrap();
                        let mut xl = collect_folder(&name);
                        let name = xl.pop().unwrap();
                        let name = name.replace(&xl.pop().unwrap(), "");
                        let temp = temp.pop().unwrap();
                        println!("====================");
                        let result = format!("{}{}", temp, name);
                        println!("FINAL:{}", result);
                        if let Err(_) = correct_path(&temp) {
                            DirBuilder::new().recursive(true).create(&temp).unwrap();
                            return file_write(&result, text, Flag::New);
                        } else {
                            let temp = correct_path(&temp).unwrap();
                            let result = format!("{}{}", temp, name);
                            file_write(&result, text, Flag::New).unwrap();
                        }
                    }),
                    ErrorKind::PermissionDenied => {
                        panic!("Permission Denied");
                    }
                    _ => panic!("other errors"),
                },
            },
        },
        Flag::New => match File::create(path) {
            Ok(_) => {
                println!("path new :{}", path);
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(path)
                    .unwrap()
                    .write_all(text.as_bytes())
            }
            Err(err) => return Err(err.kind().into()),
        },
        Flag::Old => OpenOptions::new()
            // .write(true)
            .append(true)
            .open(path)
            .unwrap()
            .write_all(text.as_bytes()),
    }
}
