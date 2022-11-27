use qfile::{file_read, file_write, Flag};
// #[test]
// fn check_file_read() {
//     assert_eq!(file_read("./files/read.txt").unwrap(), "Very well :D");
// }

#[test]
fn check_file_write() {
    let path = "./files/write.txt";
    let text = "CHEBUBELE";
    file_write(path, text, Flag::Auto).unwrap();
    assert_eq!(file_read(path).unwrap(), "CHEBUBELE");
}
// #[test]
// fn check_auto() {
//     file_write(".", "test", Flag::Old);

// }
