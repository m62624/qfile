use qfile::{file_write, Flag};

#[test]
#[ignore]
fn file_write_test_new_path() {
    let path = "./Polygon/NewPath1/file_new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}
#[test]
fn file_write_test_new_file_in_old_folder(){
    let path = "./Polygon/oldFolder1/file_new.txt";
    file_write(path, "ok", Flag::Auto).unwrap();
}