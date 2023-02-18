use super::{Directory, PathBuf};
// use crossbeam::channel::{SendError, Sender};
use rayon::prelude::*;
use regex::Regex;
use std::sync::mpsc::{self, Receiver, SendError, Sender};
use walkdir::WalkDir;
pub fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
    place: Directory,
    name: T,
    follow_link: bool,
    sender: Sender<PathBuf>,
) -> Result<(), SendError<PathBuf>> {
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
    // for element in paths {
    // println!("{:#?}", &paths);
    // println!("size parallel item:{:#?}", paths.par_iter());
    rayon::spawn(move || {
        paths
            .par_iter()
            .for_each_with(sender.clone(), |sender, element| {
                WalkDir::new(element)
                    .follow_links(follow_link)
                    .into_iter()
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
        find_paths(Directory::Everywhere, "pagefile.sys", false, tx).unwrap();
        let thread1 = thread::spawn(|| {
            for path in rx {
                println!("{}", path.display().to_string());
            }
        });
        for i in 0..10_000 {
            thread::sleep(time::Duration::from_secs(1));
            println!("main thread counter: {}", i);
        }
        thread1.join().unwrap();
    }
}
