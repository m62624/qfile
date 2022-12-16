use qfile::QFilePack;
use std::{fs, io::Read};

fn get_paths(element: usize) -> String {
    let mut data = String::new();
    fs::File::open("./tests/Paths")
        .unwrap()
        .read_to_string(&mut data)
        .unwrap();
    let data: Vec<&str> = data.as_str().split("\n").collect();
    match data.get(element) {
        Some(path) => path.to_string(),
        None => panic!("ERROR PATHS"),
    }
}
fn delete_item(path: &str) {
    if let Err(_) = fs::remove_dir_all(path) {
        dbg!("removed");
    }
}

#[cfg(target_family = "unix")]
#[test]
#[should_panic]
fn check_access_api() {
    let mut file = QFilePack::add_path("");

    file.write("").unwrap();
    file.read().unwrap();
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_1_u() {
    //=========================================
    let path = &get_paths(1);
    delete_item(&get_paths(0));
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("ok").unwrap();

    assert_eq!(file.read().unwrap(), "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_2_u() {
    //=========================================
    let path = &get_paths(2);
    delete_item(&get_paths(0));
    //=========================================

    let mut file = QFilePack::add_path(path);

    file.write("ok").unwrap();
    file.write("ok").unwrap();

    assert_eq!(file.read().unwrap(), "okok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_3_u() {
    //=========================================
    use std::path::Path;
    let path = &get_paths(3);
    let delete_i = &mut get_paths(3);
    delete_i.truncate(36);
    delete_item(delete_i);
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("").unwrap();

    assert!(Path::new("./Polygon/OldFolder/NewFolder-test3/file.txt").exists());
}
#[cfg(target_family = "unix")]
#[test]
fn test_write_4_u() {
    //=========================================
    let path = &get_paths(4);
    if let Err(_) = fs::remove_file(path) {
        dbg!("removed");
    }
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("fileroot").unwrap();

    assert_eq!(file.read().unwrap(), "fileroot");
}
#[cfg(target_family = "unix")]
#[test]
#[should_panic]
fn test_write_5_u() {
    //=========================================
    let path = &get_paths(5);
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("").unwrap();
}
//===============================================================================
#[cfg(target_family = "windows")]
#[test]
fn test_write_1_w() {
    //=========================================
    let path = &get_paths(6);
    delete_item(&get_paths(0));
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("ok").unwrap();

    assert_eq!(file.read().unwrap(), "ok");
}
#[cfg(target_family = "windows")]
#[test]
fn test_write_2_w() {
    //=========================================
    let path = &get_paths(7);
    delete_item(&get_paths(0));
    //=========================================

    let mut file = QFilePack::add_path(path);

    file.write("ok").unwrap();
    file.write("ok").unwrap();

    assert_eq!(file.read().unwrap(), "okok");
}
#[cfg(target_family = "windows")]
#[test]
fn test_write_3_w() {
    //=========================================
    use std::path::Path;
    let path = &get_paths(8);
    let delete_i = &mut get_paths(8);
    delete_i.truncate(40);
    delete_item(delete_i);
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("").unwrap();

    assert!(Path::new(".\\Polygon\\OldFolder\\NewFolder-test3\\file.txt").exists());
}
#[cfg(target_family = "windows")]
#[test]
fn test_write_4_w() {
    //=========================================
    let path = &get_paths(9);
    if let Err(_) = fs::remove_file(path) {
        dbg!("removed");
    }
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("fileroot").unwrap();

    assert_eq!(file.read().unwrap(), "fileroot");
}
#[cfg(target_family = "windows")]
#[test]
#[should_panic]
fn test_write_5_w() {
    //=========================================
    let path = &get_paths(10);
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("").unwrap();
}
#[cfg(target_family = "windows")]
#[test]
fn test_write_6_w() {
    //=========================================
    let path = &get_paths(11);
    match fs::File::open(path) {
        Ok(mut x) => {
            let mut text = String::new();
            x.read_to_string(&mut text).unwrap();
            if text == "delete me" {
                return fs::remove_file(path).unwrap();
            }
            panic!("ERROR: name match");
        }
        Err(_) => {
            dbg!("removed");
        }
    }
    //=========================================

    let mut file = QFilePack::add_path(path);
    file.write("delete me").unwrap();

    assert_eq!(file.read().unwrap(), "delete me")
}
