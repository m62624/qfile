use quick_template_file::{file_read, file_write, Flag};
#[test]
fn check_file_read() {
    assert_eq!(file_read("./files/read.txt").unwrap(), "Very well :D");
}

#[test]
fn check_file_write() {
    let text = "CHEBUBELE";
    file_write("./files/read.txt", text, Flag::Auto).unwrap();
    assert_eq!(file_read("./files/read.txt").unwrap(), "CHEBUBELE");
}
