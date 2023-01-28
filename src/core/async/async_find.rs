use super::super::RootDirectory;
use walkdir::WalkDir;
pub async fn async_find_paths<T: AsRef<str> + Send + Sync>(
    place: RootDirectory<T>,
    file_name: T,
    symlink: bool,
) -> Option<Vec<async_std::path::PathBuf>> {
    let mut paths: (Vec<String>, Vec<async_std::path::PathBuf>) =
        (Default::default(), Default::default());
    match place {
        RootDirectory::ThisPlace(root_d) => {
            paths.0.push(root_d.as_ref().to_string());
        }
        RootDirectory::Everywhere => {
            if cfg!(unix) {
                paths.0.push("/".to_string());
            }
            if cfg!(windows) {
                for disk in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>() {
                    let temp = format!("{}:\\", disk);
                    if async_std::path::PathBuf::from(&temp).exists().await {
                        paths.0.push(temp.to_string());
                    }
                }
            }
        }
    }
    for element in paths.0 {
        for entry in WalkDir::new(element)
            .follow_links(symlink)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry
                .path()
                .display()
                .to_string()
                .to_lowercase()
                .contains(&file_name.as_ref().to_string().to_lowercase())
            {
                paths.1.push(entry.path().to_path_buf().into())
            }
        }
    }
    if paths.1.is_empty() {
        return None;
    }
    return Some(paths.1);
}
