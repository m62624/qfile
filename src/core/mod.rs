mod read;
use crate::dpds_path::io::{self, ErrorKind};
use crate::dpds_path::File;
use std::env;

pub struct Pack<'a> {
    possible_directories: String,
    user_path: &'a str,
    os: &'a str,
}

//======================================================
impl<'a> Pack<'a> {
    
}
fn os_check<'a>() -> &'a str {
    env::consts::OS
}
