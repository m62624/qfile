use qfile::QPack;
#[cfg(target_family = "unix")]
#[test]
fn test_read_1() {
    let file = QPack::add_path("./Polygon/test-1.txt");
    let file = file.read().unwrap();
    assert_eq!(file, "ok");
}
