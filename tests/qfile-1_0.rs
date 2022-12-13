use qfile::QFilePack;
#[test]
#[should_panic]
fn test_qfile_api() {
    let file = QFilePack::add_path("");
    {
        let file1 = file.read().unwrap();
    }
    let file2 = file.read().unwrap();
}
