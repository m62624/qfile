use qfile::QFilePack;
use rand::Rng;
use std::fs;
use std::iter;
struct TestFolder {
    folder: String,
}
impl TestFolder {
    fn generate(len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-";
        let mut rng = rand::thread_rng();
        let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
        iter::repeat_with(one_char).take(len).collect()
    }
    fn new(first_path: &str) -> Self {
        TestFolder {
            folder: format!("{}-{}", first_path, Self::generate(15)),
        }
    }
}

fn delete_item(path: &str) {
    if let Err(_) = fs::remove_dir_all(path) {
        dbg!("removed");
    }
}
fn pwmf(main_folder: &str, path: &str) -> String {
    format!("{}{}", main_folder, path)
}
//===========================================(Unix)================================================
#[cfg(target_family = "unix")]
#[test]
fn unix_test_path_0_part1() {
    let main_folder = TestFolder::new(".Polygon").folder;
    let path = pwmf(&main_folder, "/file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    if let Ok(()) = unix_test_path_0_part2(&path) {
        assert_eq!(file.read().unwrap(), "okok");
        delete_item(&main_folder);
        return;
    }
    delete_item(&main_folder);
    panic!(":: ERROR - part2 from first_slash_add1");
}
fn unix_test_path_0_part2(path: &str) -> Result<(), std::io::Error> {
    QFilePack::add_path(path).write("ok")
}

#[cfg(target_family = "unix")]
#[test]
//..folder
fn unix_test_path_1() {
    let main_folder = TestFolder::new("../delete me").folder;
    let path = pwmf(&main_folder, "/file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}

#[cfg(target_family = "unix")]
#[test]
#[should_panic]
// root folder
fn unix_test_path_2() {
    QFilePack::add_path("/usr/invalid_file.txt")
        .write("delete this file ")
        .unwrap();
}

#[cfg(target_family = "unix")]
#[test]
// folder
fn unix_test_path_3() {
    let main_folder = TestFolder::new("Polygon").folder;
    let path = pwmf(&main_folder, "/a/b/c/file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}

#[cfg(target_family = "unix")]
#[test]
// folder
fn unix_test_path_4() {
    let main_folder = TestFolder::new("Polygon").folder;
    let path = pwmf(&main_folder, "/a/B/c/file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("").unwrap();
    let find_path = format!("{}{}", main_folder.to_lowercase(), "/A/B/c/file.txt");
    let mut find = QFilePack::add_path(&find_path);
    assert_eq!(find.cache_path(), format!("./{}", path));
    delete_item(&main_folder);
}
#[cfg(target_family = "unix")]
#[test]
// folder
fn unix_test_path_5() {
    let main_folder = TestFolder::new("./Polygon").folder;
    let path = pwmf(&main_folder, "/a/b/c/file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}
//===========================================(WINDOWS)================================================
#[cfg(target_family = "windows")]
#[test]
fn windows_test_path_0_part1() {
    let main_folder = TestFolder::new(".Polygon").folder;
    let path = pwmf(&main_folder, "\\file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    if let Ok(()) = windows_test_path_0_part2(&path) {
        assert_eq!(file.read().unwrap(), "okok");
        delete_item(&main_folder);
        return;
    }
    delete_item(&main_folder);
    panic!(":: ERROR - part2 from first_slash_add1");
}
fn windows_test_path_0_part2(path: &str) -> Result<(), std::io::Error> {
    QFilePack::add_path(path).write("ok")
}

#[cfg(target_family = "windows")]
#[test]
//..folder
fn windows_test_path_1() {
    let main_folder = TestFolder::new("..\\delete me").folder;
    let path = pwmf(&main_folder, "\\file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}

#[cfg(target_family = "windows")]
#[test]
#[should_panic]
// root folder
fn windows_test_path_2() {
    QFilePack::add_path("C:\\Windows\\System32\\invalid_file.txt")
        .write("delete this file ")
        .unwrap();
}

#[cfg(target_family = "windows")]
#[test]
// folder
fn windows_test_path_3() {
    let main_folder = TestFolder::new("Polygon").folder;
    let path = pwmf(&main_folder, "\\a\\b\\cfile.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}

#[cfg(target_family = "windows")]
#[test]
// folder
fn windows_test_path_4() {
    let main_folder = TestFolder::new("D:\\Polygon").folder;
    let path = pwmf(&main_folder, "\\a\\b\\cfile.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("ok").unwrap();
    assert_eq!(file.read().unwrap(), "ok");
    delete_item(&main_folder);
}

#[cfg(target_family = "windows")]
#[test]
// folder
fn unix_test_path_5() {
    let main_folder = TestFolder::new("Polygon").folder;
    let path = pwmf(&main_folder, "\\a\\B\\c\\file.txt");
    let mut file = QFilePack::add_path(&path);
    file.write_only_new("").unwrap();
    let find_path = format!("{}{}", main_folder.to_lowercase(), "\\A\\B\\c\\file.txt");
    let mut find = QFilePack::add_path(&find_path);
    assert_eq!(find.cache_path(), path);
    delete_item(&main_folder);
}
