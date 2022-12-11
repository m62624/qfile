use crate::core::os_check;
use crate::core::read::only_for_crate;
use crate::dpds_path::fs;
use crate::dpds_path::Path;
use crate::dpds_path::{io, Regex};
pub fn collect_folder(path: &str) -> Vec<String> {
    let os_v = os_check();
    let os_v = if os_v == "linux" || os_v == "macos" {
        //r"^(?:[\./]|\.\./|\./|[\./]?)|(?:(?:\.\./|\./|[\./])?[^/]*)"
        //^(?:\.+[^\/]+|\.\.|(?:\.\/)|\/)|[^\/]+|\/
        Regex::new(r"(?:\./|\.\.|(?:\.\./|\./|[\./])?[^/]*)").unwrap()
    } else if os_v == "windows" {
        //r"^(?:.:\\|\.\.\\|\.\\|[\.\\]?)|(?:(?:\.\.\\|\.\\|[\.\\])?[^\\]*)"
        Regex::new(r"^(?:[^\.\\]+|.:\\|\.\.\\|\.\\|[\.\\]?)|(?:(?:\.\.\\|\.\\|[\.\\])?[^\\]*)").unwrap()
    } else {
        panic!("OS not defined");
    };

    let (mut folders, mut i, rgx) = (Vec::new(), 1, os_v);
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

pub fn correct_path(path: &str) -> Result<String, io::Error> {
    let mut user_paths = collect_folder(path);
    for i in 0..user_paths.len() {
        let mut really_paths = files(&user_paths[i]);
        for j in 0..really_paths.len() {
            if user_paths
                .get(i + 1)
                .unwrap_or(&user_paths.get(i).unwrap())
                .to_lowercase()
                == really_paths[j].to_lowercase()
            {
                user_paths[i + 1] = really_paths.remove(j);
                break;
            }
        }
    }
    let (value_1, value_2) = (
        &user_paths[user_paths.len() - 1],
        &user_paths[user_paths.len() - 2],
    );
    match only_for_crate::file_read(&value_1) {
        Ok(_) => Ok(user_paths.pop().unwrap()),
        Err(err) => {
            if let true = Path::new(&value_1).is_dir() {
                return Ok(user_paths.pop().unwrap());
            } else {
                let mut with_slash = value_2.to_lowercase();
                if os_check() == "linux" || os_check() == "macos" {
                    with_slash.push('/');
                } else if os_check() == "windows" {
                    with_slash.push('\\');
                }
                if value_1.to_lowercase() == with_slash {
                    return Ok(value_2.to_string());
                }
            }
            return Err(err);
        }
    }
}
#[test]
fn correct_path_without_slash() {
    assert_eq!(
        correct_path("Polygon/correct2").unwrap().as_str(),
        "./Polygon/correct2"
    );
}
#[test]
fn correct_path_with_file_test() {
    assert_eq!(
        correct_path("./Polygon/correctPath1/FILE1.txt")
            .unwrap()
            .as_str(),
        "./Polygon/CorrectPath1/file1.txt"
    );
}
#[test]
#[should_panic]
// #[ignore]
fn correct_path_with_file_test_panic() {
    assert_eq!(
        correct_path(".Polygon/correctPath1/unknown.txt")
            .unwrap()
            .as_str(),
        "./Polygon/correctPath1/unknown.txt"
    );
}
#[test]
fn correct_path_without_file_test() {
    assert_eq!(
        correct_path("./Polygon/correctPath2/").unwrap().as_str(),
        "./Polygon/CorrectPath2"
    );
}
#[test]
fn correct_path_with_slash_test() {
    dbg!("with slash : {}", correct_path("./Polygon/").unwrap());
    assert_eq!(correct_path("./Polygon/").unwrap().as_str(), "./Polygon/");
}
#[test]
#[should_panic]
// #[ignore]
fn correct_path_without_file_test_panic() {
    assert_eq!(
        correct_path("./Polygon/correct2").unwrap().as_str(),
        "./Polygon/correct2"
    );
}
#[test]
fn check_system() {
    dbg!("{}", os_check());
}
