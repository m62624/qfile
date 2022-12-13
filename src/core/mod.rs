mod read;
mod write;
use crate::dpds_path::{fs, io, lazy_static, ErrorKind, File, Regex, __Deref};
use std::env;
#[derive(Debug)]
pub struct QFilePack<'a> {
    //================
    possible_directories: Vec<String>,
    request_directories: Vec<String>,
    //================
    user_path: &'a str,
    correct_path: &'a str,
    //================
    os: &'a str,
}

//======================================================
impl<'a> QFilePack<'a> {
    pub fn add_path(path: &'a str) -> Self {
        QFilePack {
            possible_directories: Default::default(),
            request_directories: Default::default(),
            user_path: path,
            correct_path: Default::default(),
            os: env::consts::OS,
        }
    }

    fn way_step_by_step(&mut self) {
        let mut items = |rgx: &Regex, path: &str| {
            let (mut folders, mut i) = (Vec::new(), 1);
            let mut captures = rgx.captures_iter(path);
            folders.push(captures.next().unwrap()[0].to_string());
            for element in captures {
                folders.push(format!("{}{}", folders[i - 1], &element[0]));
                i += 1;
            }
            self.request_directories = folders;
        };
        match self.os {
            "linux" | "macos" => {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r"(?:\./|\.\.|(?:\.\./|\./|[\./])?[^/]*)").unwrap();
                }
                return items(RE.deref(), self.user_path);
            }
            "windows" => {
                lazy_static! {
                    static ref RE: Regex =
                        Regex::new(r"(?:.:\\|\.\\|\.\.|(?:\.\.\\|\.\\|[\.\\])?[^\\]*)").unwrap();
                }
                return items(RE.deref(), self.user_path);
            }
            _ => {
                panic!(":: unsupported system ::")
            }
        };
    }

    fn correct_path(&mut self) {
        self.way_step_by_step();
        let request_directories = &self.request_directories;
        for user_i in 0..request_directories.len() {
            // println!("{}", request_directories[user_i]);
            
        }
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
fn directory_contents(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(paths) = fs::read_dir(path) {
        for items in paths {
            if let Ok(items) = items {
                files.push(items.path().display().to_string());
            }
        }
    }
    // self.possible_directories = files;
    return files;
}

//=====================================(tests)=====================================
#[cfg(target_family = "unix")]
#[test]
fn test_way_step_by_step() {
    let mut temp = QFilePack::add_path("./Polygon/Don't delete/test-1.txt");
    temp.way_step_by_step();
    assert_eq!(
        temp.request_directories,
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
    dbg!(directory_contents("./Polygon/Don't delete"));
    assert_eq!(
        directory_contents("./Polygon/Don't delete"),
        vec![
            "./Polygon/Don't delete/test-1.txt",
            "./Polygon/Don't delete/temp3.txt",
            "./Polygon/Don't delete/temp1.txt",
            "./Polygon/Don't delete/tempFolder",
            "./Polygon/Don't delete/temp2.txt",
        ]
    )
}
#[test]
fn test_correct_path() {
    let mut temp = QFilePack::add_path("./polygon/Read/test-1.txt");
    temp.correct_path();
    assert_eq!(true, true);
}
