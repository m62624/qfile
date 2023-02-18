use super::{Directory, PathBuf};
// use crossbeam::channel::{SendError, Sender};
use rayon::prelude::*;
use regex::Regex;
use std::sync::mpsc::{self, SendError, Sender};
use walkdir::WalkDir;
pub fn find_paths<T: AsRef<str> + Send + Sync + 'static, E: AsRef<str> + Send + Sync + 'static>(
    place: Directory<T>,
    name: T,
    excluded_dirs: Option<Vec<E>>,
    follow_link: bool,
    sender: Sender<PathBuf>,
) -> Result<(), SendError<PathBuf>> {
    rayon::spawn(move || {
        let mut paths: Vec<String> = Default::default();
        match place {
            Directory::ThisPlace(root_d) => {
                paths = root_d
                    .iter()
                    .map(|x| x.as_ref().to_owned())
                    .collect::<Vec<String>>()
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
        let excluded_dirs = match excluded_dirs {
            Some(values) => values
                .iter()
                .map(|x| x.as_ref().to_owned())
                .collect::<Vec<String>>(),
            None => Default::default(),
        };
        paths
            .par_iter()
            .for_each_with(sender.clone(), |sender, element| {
                WalkDir::new(element)
                    .follow_links(follow_link)
                    .into_iter()
                    .filter_entry(|entry| {
                        !excluded_dirs
                            .iter()
                            .any(|excl| entry.path().display().to_string().contains(excl))
                    })
                    .filter_map(|e| e.ok())
                    .collect::<Vec<_>>()
                    .par_iter()
                    .for_each_with(sender.clone(), |sender, entry| {
                        if entry
                            .path()
                            .display()
                            .to_string()
                            .to_lowercase()
                            .contains(&name.as_ref().to_string().to_lowercase())
                        {
                            if let Err(err) = sender.send(entry.path().to_path_buf().into()) {
                                panic!("{}", err);
                            }
                        }
                    });
            });
    });
    Ok(())
}

#[cfg(test)]
mod test_find {
    use super::*;
    use std::{thread, time};
    #[test]
    fn check_find_path() {
        let (tx, rx) = mpsc::channel();
        let excludedir = vec!["/run/media", "/bin"];
        find_paths(
            Directory::ThisPlace(vec!["/"]),
            "2023-02-17 21-12-11",
            Some(excludedir),
            false,
            tx,
        )
        .unwrap();
        for path in rx {
            println!("{}", path.display().to_string());
        }
    }
}
