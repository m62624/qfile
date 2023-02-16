use super::super::Directory;
use regex::Regex;
use std::path::PathBuf;
use std::sync::mpsc::{SendError, Sender};
use std::thread;
use threadpool::ThreadPool;
use walkdir::WalkDir;
pub fn find_paths<T: AsRef<str> + Send + Sync + 'static>(
    place: Directory,
    name: T,
    follow_link: bool,
    sender: Sender<PathBuf>,
) -> Result<(), SendError<PathBuf>> {
    todo!()
}
