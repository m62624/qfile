mod read;
mod write;
use crate::dpds_path::{fs, io, lazy_static, ErrorKind, File, Regex, __Deref};
use std::env;
pub struct QFilePack<'a> {
    possible_directories: Vec<String>,
    user_path: &'a str,
    correct_path: &'a str,
    os: &'a str,
}

//======================================================
impl<'a> QFilePack<'a> {
    pub fn add_path(path: &'a str) -> Self {
        QFilePack {
            possible_directories: Default::default(),
            user_path: path,
            correct_path: Default::default(),
            os: env::consts::OS,
        }
    }
    fn directory_contents(&mut self) {
        let mut files: Vec<String> = Vec::new();
        if let Ok(paths) = fs::read_dir(self.user_path) {
            for items in paths {
                if let Ok(items) = items {
                    files.push(items.path().display().to_string());
                }
            }
        }
        self.possible_directories = files;
    }
}
fn get_file(path: &str) -> Result<File, io::Error> {
    match File::open(path) {
        Ok(file) => Ok(file),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => Err(err),
            ErrorKind::PermissionDenied => Err(err),
            ErrorKind::InvalidData => Err(err),
            _ => panic!(":: other errors ::"),
        },
    }
}

fn way_step_by_step(os: &str, path: &str) -> Vec<String> {
    let items = |rgx: &Regex, path: &str| -> Vec<String> {
        let (mut folders, mut i) = (Vec::new(), 1);
        let mut captures = rgx.captures_iter(path);
        folders.push(captures.next().unwrap()[0].to_string());
        for element in captures {
            folders.push(format!("{}{}", folders[i - 1], &element[0]));
            i += 1;
        }
        return folders;
    };
    match os {
        "linux" | "macos" => {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"(?:\./|\.\.|(?:\.\./|\./|[\./])?[^/]*)").unwrap();
            }
            return items(RE.deref(), path);
        }
        "windows" => {
            lazy_static! {
                static ref RE: Regex =
                    Regex::new(r"(?:.:\\|\.\\|\.\.|(?:\.\.\\|\.\\|[\.\\])?[^\\]*)").unwrap();
            }
            return items(RE.deref(), path);
        }
        _ => {
            panic!(":: unsupported system ::")
        }
    };
}
//=====================================(tests)=====================================
#[cfg(target_family = "unix")]
#[test]
fn test_way_step_by_step() {
    assert_eq!(
        way_step_by_step("linux", "./Polygon/Don't delete/test-1.txt"),
        vec![
            "./",
            "./Polygon",
            "./Polygon/Don't delete",
            "./Polygon/Don't delete/test-1.txt"
        ]
    );
}
#[cfg(target_family = "unix")]
#[test]
fn test_path_content() {
    let mut temp = QFilePack::add_path("./Polygon/Don't delete");
    temp.directory_contents();
    assert_eq!(
        temp.possible_directories,
        vec![
            "./Polygon/Don't delete/test-1.txt",
            "./Polygon/Don't delete/temp3.txt",
            "./Polygon/Don't delete/temp1.txt",
            "./Polygon/Don't delete/tempFolder",
            "./Polygon/Don't delete/temp2.txt",
        ]
    )
}
fn test_regex_1() {}
