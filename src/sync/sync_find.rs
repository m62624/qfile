use super::super::Directory;
use regex::Regex;
use std::sync::mpsc::{SendError, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use walkdir::WalkDir;

pub fn find_paths<T: AsRef<str> + Send + Sync + Copy + 'static>(
    place: Directory,
    name: T,
    follow_link: bool,
    sender: Sender<std::path::PathBuf>,
) -> Result<(), SendError<std::path::PathBuf>> {
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

    let thread_count = std::cmp::min(num_cpus::get(), paths.len());
    let arc_sender = Arc::new(Mutex::new(sender));
    let mut join_handles = vec![];
    for path in paths {
        let arc_sender_clone = Arc::clone(&arc_sender);
        let join_handle = thread::spawn(move || {
            for entry in WalkDir::new(path)
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
                    let result = arc_sender_clone
                        .lock()
                        .unwrap()
                        .send(entry.path().to_path_buf().into());
                    if let Err(error) = result {
                        eprintln!("Error sending path: {}", error);
                    }
                }
            }
        });
        join_handles.push(join_handle);
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
    Ok(())
}
