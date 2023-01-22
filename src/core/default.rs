use super::{Flag, OptionCodeFile, QFilePath, OS_QFILE};
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
impl<'a> Default for QFilePath<'a> {
    fn default() -> Self {
        QFilePath {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: Default::default(),
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
        }
    }
    
}


