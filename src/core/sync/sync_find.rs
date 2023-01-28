use super::super::RootDirectory;
use std::error::Error;
use std::sync::mpsc::{self, SendError};
use std::thread;
use walkdir::WalkDir;
pub fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
    sender: mpsc::Sender<Option<Vec<std::path::PathBuf>>>,
    place: RootDirectory<T>,
    file_name: T,
) -> Result<(), SendError<Option<Vec<std::path::PathBuf>>>> {
    thread::spawn(
        move || -> Result<(), SendError<Option<Vec<std::path::PathBuf>>>> {
            let mut paths: (Vec<String>, Vec<std::path::PathBuf>) =
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
                            if std::path::PathBuf::from(&temp).exists() {
                                paths.0.push(temp.to_string());
                            }
                        }
                    }
                }
            }
            for element in paths.0 {
                for entry in WalkDir::new(element).into_iter().filter_map(|e| e.ok()) {
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
                return Ok(sender.send(None)?);
            }
            return Ok(sender.send(Some(paths.1))?);
        },
    );
    Ok(())
}
