mod dpds_path;
pub use crate::core::read::file_read;
pub use crate::core::write::{file_write, Flag};
mod core {
    use crate::dpds_path::io::{self, ErrorKind};
    use crate::dpds_path::File;
    fn get_file(path: &str) -> Result<File, io::Error> {
        match File::open(path) {
            Ok(file) => Ok(file),
            Err(err) => match err.kind() {
                ErrorKind::NotFound => return Err(err.kind().into()),
                ErrorKind::PermissionDenied => return Err(err.kind().into()),
                _ => panic!("::other error::"),
            },
        }
    }
    pub mod read {

        use crate::dpds_path::io::{self, Read};

        use super::get_file;

        pub fn file_read(path: &str) -> Result<String, io::Error> {
            let mut text = String::new();
            if let Err(err) = get_file(path).unwrap().read_to_string(&mut text) {
                return Err(err.into());
            }
            Ok(text)
        }
    }

    pub mod write {
        // use crate::;

        use crate::dpds_path::io::{self, Write};
        use crate::dpds_path::File;

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
                    Ok(_) => get_file(path).unwrap().by_ref().write_all(text.as_bytes()),
                    Err(err) => return Err(err.kind().into()),
                },
                Flag::Old => get_file(path).unwrap().by_ref().write_all(text.as_bytes()),
            }
        }
    }
}
