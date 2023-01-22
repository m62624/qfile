use super::QFilePath;
impl<'a> Drop for QFilePath<'a> {
    fn drop(&mut self) {}
}
