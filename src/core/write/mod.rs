pub mod files;
use std::fmt::format;

use self::files::{collect_folder, correct_path};
use super::get_file;
use crate::dpds_path::io::{self, ErrorKind, Write};
use crate::dpds_path::{DirBuilder, File, OpenOptions};

pub enum Flag {
    New,
    Auto,
    Old,
}

// #[cfg(test)]
// mod test {
//     use crate::core::write::collect_folder;
//     #[test]
//     fn regex_check() {
//         println!(
//             "{:#?}",
//             collect_folder("../files/folder.d/folder.mr.d/file.txt")
//         );
//         println!(
//             "{:#?}",
//             collect_folder("/files/folder.d/folder.mr.d/file.txt")
//         );
//         println!(
//             "{:#?}",
//             collect_folder("./files/folder.d/folder.mr.d/file.txt")
//         )g
//     }
// }

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
                        // println!("mb name {}", name);
                        let temp = temp.pop().unwrap();
                        // let temp = temp.remove(temp.len() - 2);
                        // println!("mb new folder {}", temp);

                        if let Err(_) = correct_path(&temp) {
                            DirBuilder::new().recursive(true).create(&temp).unwrap();
                            return file_write(&name, text, Flag::New);
                            // panic!("last folder problem,{}", err);
                        } else {
                            // println!("та же самая папка: {}", &correct_path(&temp).unwrap());
                            let result = format!("{}{}", correct_path(&temp).unwrap(), name);
                            file_write(&result, text, Flag::New).unwrap();
                        }
                    }),
                    ErrorKind::PermissionDenied => {
                        panic!("Permission Denied");
                    }
                    _ => panic!("other errors"),
                }, // Err(_) => {
                   //     let mut temp = collect_folder(path);
                   //     let temp = temp.remove(temp.len() - 2);
                   //     println!(":: temp {}", temp);
                   //     DirBuilder::new().recursive(true).create(temp).unwrap();
                   //     return file_write(path, text, Flag::New);
                   // }
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
