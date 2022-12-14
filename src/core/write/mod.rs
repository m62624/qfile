use crate::core::QFilePack;
use crate::core::{get_file, Flag};
use crate::dpds_path::io::{self, Write};
use crate::dpds_path::{DirBuilder, ErrorKind, File, OpenOptions};

impl<'a> QFilePack<'a> {
    pub fn write(&mut self, text: &'a str) -> Result<(), io::Error> {
        let os = self.os;
        if self.update_path {
            match self.os {
                "linux" | "macos" => {
                    if self.correct_path.is_empty() {
                        self.correct_path = format!("{}{}{}", self.user_path, "/", self.file_name)
                    } else {
                        self.correct_path =
                            format!("{}{}{}", self.correct_path.clone(), "/", self.file_name)
                    }
                }
                "windows" => {
                    self.correct_path =
                        format!("{}{}{}", self.correct_path.clone(), "\\", self.file_name)
                }
                _ => panic!(),
            }
        }
        match self.flag {
            Flag::Auto => match get_file(self.cache_path()) {
                Ok(_) => {
                    self.flag = Flag::Old;
                    return self.write(text);
                }
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        self.cache_path().to_string();
                        let fullpath = self.user_path;
                        let filename = match os {
                            "linux" | "macos" => fullpath.rsplit_once("/").unwrap().1,
                            "windows" => fullpath.rsplit_once("\\").unwrap().1,
                            _ => panic!(),
                        };
                        let path_without_file = {
                            let temp = fullpath.rsplit_once(filename).unwrap().0;
                            let (first, _) = temp.split_at(temp.len() - 1);
                            first
                        };
                        {
                            self.user_path = path_without_file;
                            self.correct_path();
                            self.update_path = true;
                            self.file_name = filename;
                            self.flag = Flag::New;
                        }
                        DirBuilder::new()
                            .recursive(true)
                            .create(self.cache_path())
                            .unwrap();
                        return self.write(text);
                    }
                    ErrorKind::PermissionDenied => {
                        panic!()
                    }
                    _ => panic!("other errors"),
                },
            },

            Flag::New => match File::create(self.cache_path()) {
                Ok(_) => {
                    self.update_path = false;
                    self.flag = Flag::Auto;
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(self.cache_path())
                        .unwrap()
                        .write_all(text.as_bytes())
                }
                Err(err) => return Err(err),
            },
            Flag::Old => {
                self.flag = Flag::Auto;
                OpenOptions::new()
                    .append(true)
                    .open(self.cache_path())
                    .unwrap()
                    .write_all(text.as_bytes())
            }
        }
    }
}
//=====================================(tests)=====================================
#[cfg(target_family = "unix")]
#[test]
fn test_write_1() {
    let mut file = QFilePack::add_path("./polygon/write/new.txt");
    file.write("ok").unwrap();
    let data = file.read().unwrap();
    assert_eq!(data, "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_2() {
    let mut file = QFilePack::add_path("./Polygon/ru_Папка/new.txt");
    file.write("ok").unwrap();
    let data = file.read().unwrap();
    assert_eq!(data, "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_3() {
    let mut file = QFilePack::add_path("./new.txt");
    file.write("ok").unwrap();
    let data = file.read().unwrap();
    assert_eq!(data, "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_4() {
    let mut file = QFilePack::add_path("./Polygon/write/new-4.txt");
    file.write("oldata").unwrap();
    file.write("newdata").unwrap();
    let data = file.read().unwrap();
    assert_eq!(data, "oldatanewdata");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_5() {
    let mut file = QFilePack::add_path("root.txt");
    file.write("ok").unwrap();
    let data = file.read().unwrap();
    assert_eq!(data, "ok");
}
