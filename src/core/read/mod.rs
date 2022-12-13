use crate::core::QPack;
use crate::dpds_path::{fs, io, lazy_static, ErrorKind, File, Read, Regex};
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err),
            ErrorKind::PermissionDenied => Err(err),
            _ => panic!(":: other errors ::"),
        },
    }
}

pub fn collect_folder(os: &str, path: &str) -> Vec<String> {
    match os {
        "linux" | "macos" => {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"(?:\./|\.\.|(?:\.\./|\./|[\./])?[^/]*)").unwrap();
            }
        }
        "windows" => {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"(?:.:\\|\.\\|\.\.|(?:\.\.\\|\.\\|[\.\\])?[^\\]*)").unwrap();
            }
        }
    }
    let (mut folders, mut i, rgx) = (Vec::new(), 1, os);
    let mut captures = rgx.captures_iter(path);
    folders.push(captures.next().unwrap()[0].to_string());
    for element in captures {
        folders.push(format!("{}{}", folders[i - 1], &element[0]));
        i += 1;
    }
    return folders;
}

fn files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(paths) = fs::read_dir(path) {
        for items in paths {
            if let Ok(items) = items {
                files.push(items.path().display().to_string());
            }
        }
    }
    return files;
}

fn correct_path(os: &str, path_user: &str) -> &str {}

impl<'a> QPack<'a> {
    pub fn read(&self) -> Result<String, io::Error> {
        let mut text = String::new();
        match get_file(self.user_path) {
            Ok(mut access) => match access.read_to_string(&mut text) {
                Ok(_) => return Ok(text),
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => {
                return Err(err);
            }
        }
    }
}
