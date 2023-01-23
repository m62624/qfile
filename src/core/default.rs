use super::{
    Flag, OptionCodeFile, OptionCodePathBuf, OptionCodeRequestItems, QFilePath, QPatternPath,
};
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
impl Default for QPatternPath {
    fn default() -> Self {
        QPatternPath::DefaultPattern
    }
}
impl Default for OptionCodeRequestItems {
    fn default() -> Self {
        OptionCodeRequestItems::UnknownStatusRequestItems
    }
}
impl Default for QFilePath {
    fn default() -> Self {
        Self {
            request_items: Default::default(),
            only_file: Default::default(),
            user_path: Default::default(),
            file_name: Default::default(),
            correct_path: Default::default(),
            flag: Default::default(),
            update_path: false,
            pattern: Default::default(),
        }
    }
}
