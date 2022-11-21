use quick_template_file::file_read;
#[test]
fn check_file_read() {
    assert_eq!(file_read("./files/read.txt").unwrap(), "Very well :D");
}

#[test]
fn check_file_write() {
    assert_eq!(file_write("./files/write.txt",).unwrap(), "So good :D");
}
