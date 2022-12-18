#[cfg(target_family = "unix")]
#[test]
#[should_panic]
fn test_read_2() {
    use crate::QFilePack;

    let mut file = QFilePack::add_path("");
    {
        let _file1 = file.read().unwrap();
    }
    let _file2 = file.read().unwrap();
}
#[cfg(target_family = "unix")]
#[test]
fn test_read_3() {
    use crate::QFilePack;
    use std::path::Path;

    //==============================================================
    let mut file = QFilePack::add_path("./polygon/Read/test-3.txt");
    if !Path::new("./Polygon/Read/test-3.txt").exists() {
        file.write("ok").unwrap();
    }
    //==============================================================

    let data = file.read().unwrap();
    let data2 = file.read().unwrap();

    assert_eq!(data, "ok");
    assert_eq!(data2, "ok");
}
#[cfg(target_family = "unix")]
#[test]
fn test_read_1() {
    use crate::QFilePack;
    use std::path::Path;
    //==============================================================
    let mut file = QFilePack::add_path("./Polygon/READ/Test-1.txt");
    if !Path::new("./Polygon/Read/test-1.txt").exists() {
        file.write("ok").unwrap();
    }
    //==============================================================

    let data = file.read().unwrap();
    let data2 = file.read().unwrap();

    assert_eq!(data, "ok");
    assert_eq!(data2, "ok");
}
