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
    println!("{:#?}", &paths);
    paths
        .par_iter()
        .for_each_with(sender.clone(), |sender, element| {
            for entry in WalkDir::new(element)
                .follow_links(follow_link)
                .into_iter()
                .filter_map(
                    |e| e.ok(), //     match e {
                                //     Ok(item) => Some(item),
                                //     Err(_) => None,
                                // }
                )
            {
                // println!(
                //     "status: {}",
                //     entry.path().display().to_string().to_lowercase()
                // );
                if entry
                    .path()
                    .display()
                    .to_string()
                    .to_lowercase()
                    .contains(&name.as_ref().to_string().to_lowercase())
                {
                    if let Err(error) = sender.send(entry.path().to_path_buf().into()) {
                        eprintln!("Error sending path: {}", error);
                    }
                }
            }
        });

    // }
    // for element in paths {
    //     WalkDir::new(element)
    //         .follow_links(follow_link)
    //         .into_iter()
    //         .filter_map(|e| e.ok())
    //         .collect::<Vec<_>>()
    //         .par_iter()
    //         .for_each_with(sender.clone(), |sender, entry| {
    //             if entry
    //                 .path()
    //                 .display()
    //                 .to_string()
    //                 .to_lowercase()
    //                 .contains(&name.as_ref().to_string().to_lowercase())
    //             {
    //                 if let Err(err) = sender.send(entry.path().to_path_buf().into()) {
    //                     panic!("{}", err);
    //                 }
    //             }
    //         });
    // }
    // drop(sender);
    drop(sender);
    Ok(())
}
#[test]
fn check_find_path() {
    let (tx, rx) = mpsc::channel();
    find_paths(Directory::Everywhere, "pagefile.sys", false, tx).unwrap();
    for path in rx {
        println!("{}", path.display().to_string());
    }
}
