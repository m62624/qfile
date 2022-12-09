use qfile::{file_read, file_write, Flag};

#[test]
#[ignore]
fn file_write_test_new_path() {
    let path = "./Polygon/NewPath1/file_new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}
#[test]
fn file_write_test_new_file_in_old_folder() {
    let path = "./Polygon/oldFolder1/file_new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}
#[test]
fn file_write_test_new_folder_in_old_folder_with_register() {
    let path = "./Polygon/OldFolder1/new_folder_with_register/new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}
#[test]
fn file_write_test_new_folder_in_old_folder_without_register() {
    let path = "./Polygon/oldfolder1/x1/x2/new_folder_without_register/new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}
#[test]
fn file_read_test() {
    let x = file_read("./Polygon/oldFolder1/file_new.txt").unwrap();
    dbg!(x);
}

#[test]
fn file_windows_check() {
    file_write(".\\oldFolder1\\file_new.txt", "ok", Flag::Auto).unwrap();
}
#[test]
fn just_file_test_without_slash() {
    file_write("new.txt", "test new file", Flag::Auto).unwrap();
}
#[test]
fn just_file_test_with_slash() {
    file_write("/new.txt", "test new file", Flag::Auto).unwrap();
}
