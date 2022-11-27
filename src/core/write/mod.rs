pub mod files;
use self::files::correct_path;
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
                Ok(new_path) => return file_write(&new_path, text, Flag::Old),
                Err(_) => {
                    DirBuilder::new().recursive(true).create(path).unwrap();
                    return file_write(path, text, Flag::New);
                }
            },
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
            // .write(true)
            .append(true)
            .open(path)
            .unwrap()
            .write_all(text.as_bytes()),
    }
}
