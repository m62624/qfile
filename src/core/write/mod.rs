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
                Ok(new_path) => file_write(&new_path, text, Flag::New),
                Err(err) => {
                    let mut temp = collect_folder(path);
                    let temp = temp.remove(temp.len() - 2);
                    println!(":: temp {}", temp);
                    DirBuilder::new().recursive(true).create(temp).unwrap();
                    return file_write(path, text, Flag::New);
                }
            }, // Err(err) => {
               //     if let ErrorKind::NotFound = err.kind() {
               //         let mut temp = collect_folder(path);
               //         let temp = temp.remove(temp.len() - 2);
               //         println!(":: temp {}",temp);
               //         DirBuilder::new().recursive(true).create(temp).unwrap();
               //         return file_write(path, text, Flag::New);
               //     } else {
               //         panic!("other errors")
               //     }
               // }
               // Err(_) => {
               //     let mut temp = collect_folder(path);
               //     let temp = temp.remove(temp.len() - 2);
               //     println!(":: temp {}", temp);
               //     DirBuilder::new().recursive(true).create(temp).unwrap();
               //     return file_write(path, text, Flag::New);
               // }
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
