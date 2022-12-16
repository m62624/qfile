#[cfg(target_family = "unix")]
#[test]
fn test_way_step_by_step() {
    use crate::QFilePack;
    let mut temp = QFilePack::add_path("./Polygon/CorrectPath/f1.txt");
    temp.way_step_by_step();
    assert_eq!(
        temp.request_items,
        vec![
            "./",
            "./Polygon",
            "./Polygon/CorrectPath",
            "./Polygon/CorrectPath/f1.txt"
        ]
    );
}
#[cfg(target_family = "unix")]
#[test]
fn test_path_content() {
    use crate::core::directory_contents;
    assert_eq!(
        directory_contents("./Polygon/CorrectPath"),
        vec![
            "./Polygon/CorrectPath/F2.txt",
            "./Polygon/CorrectPath/f1.txt",
            "./Polygon/CorrectPath/F3.txt",
        ]
    )
}
#[cfg(target_family = "unix")]
#[test]
fn test_correct_path() {
    use crate::QFilePack;
    let mut temp = QFilePack::add_path("./polygon/correctPATH/F1.txt");
    temp.correct_path();
    assert_eq!(temp.read().unwrap(), "ok");
}
