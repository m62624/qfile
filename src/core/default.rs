use super::{Flag, OptionCodeFile, OptionCodePathBuf, QFilePath};
impl Default for OptionCodeFile {
    fn default() -> Self {
        OptionCodeFile::UnknownStatusFile
    }
}
impl Default for Flag {
    fn default() -> Self {
        Flag::Auto
    }
}
impl Default for OptionCodePathBuf {
    fn default() -> Self {
        OptionCodePathBuf::UnknownStatusPathBuf
    }
}
impl<'a> Default for QFilePath<'a> {
    fn default() -> Self {
        QFilePath {
            update_path: false,
            ..Default::default()
        }
    }
}
