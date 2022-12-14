use qfile::QFilePack;
#[cfg(target_family = "unix")]
#[test]
#[should_panic]
fn check_access_api() {
    let mut file = QFilePack::add_path("");
    file.write("").unwrap();
    file.read().unwrap();
}
#[test]
fn temp() {
    let mut qpack = QFilePack::add_path("./new_file.txt");
    let file = qpack.file().unwrap();
    assert_eq!(file.metadata().unwrap().is_dir(), true);
}
