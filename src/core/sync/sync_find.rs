use super::super::Directory;
use regex::Regex;
use std::sync::mpsc::{SendError, Sender};
use std::thread;
use walkdir::WalkDir;
pub fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
    place: Directory,
    name: T,
    follow_link: bool,
    sender: Sender<std::path::PathBuf>,
) -> Result<(), SendError<std::path::PathBuf>> {
    thread::spawn(
        move || -> Result<(), std::sync::mpsc::SendError<std::path::PathBuf>> {
            let mut paths: Vec<String> = Default::default();
            match place {
                Directory::ThisPlace(root_d) => {
                    paths.push(root_d);
                }
                Directory::Everywhere => {
                    if cfg!(unix) {
                        paths.push("/".to_string());
                    }
                    if cfg!(windows) {
                        for disk in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>() {
                            let temp = format!("{}:\\", disk);
                            if std::path::PathBuf::from(&temp).exists() {
                                paths.push(temp.to_string());
                            }
                        }
                    }
                }
            }
            for element in paths {
                for entry in WalkDir::new(element)
                    .follow_links(follow_link)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry
                        .path()
                        .display()
                        .to_string()
                        .to_lowercase()
                        .contains(&name.as_ref().to_string().to_lowercase())
                    {
                        sender.send(entry.path().to_path_buf().into())?;
                    }
                }
            }
            Ok(())
        },
    );
    Ok(())
}
pub fn find_regex_paths(
    place: Directory,
    name: Regex,
    follow_link: bool,
    sender: Sender<std::path::PathBuf>,
) -> Result<(), SendError<std::path::PathBuf>> {
    thread::spawn(
        move || -> Result<(), std::sync::mpsc::SendError<std::path::PathBuf>> {
            let mut paths: Vec<String> = Default::default();
            match place {
                Directory::ThisPlace(root_d) => {
                    paths.push(root_d);
                }
                Directory::Everywhere => {
                    if cfg!(unix) {
                        paths.push("/".to_string());
                    }
                    if cfg!(windows) {
                        for disk in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>() {
                            let temp = format!("{}:\\", disk);
                            if std::path::PathBuf::from(&temp).exists() {
                                paths.push(temp.to_string());
                            }
                        }
                    }
                }
            }
            for element in paths {
                for entry in WalkDir::new(element)
                    .follow_links(follow_link)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if name.is_match(&entry.path().display().to_string()) {
                        {
                            sender.send(entry.path().to_path_buf().into())?;
                        }
                    }
                }
            }
            Ok(())
        },
    );
    Ok(())
}
