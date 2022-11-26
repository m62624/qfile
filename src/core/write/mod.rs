use super::get_file;
use crate::dpds_path::io::{self, Write};
use crate::dpds_path::Regex;
use crate::dpds_path::{File, OpenOptions};

pub enum Flag {
    New,
    Auto,
    Old,
}
fn collect_folder(path: &str) -> Vec<String> {
    let mut folders: Vec<String> = Vec::new();
    let mut i = 1;
    let rgx = Regex::new(r"(?:../|\./|[\./])?[^/]*").unwrap();
    let mut captures = rgx.captures_iter(path);
    folders.push(captures.next().unwrap()[0].to_string());
    for element in captures {
        folders.push(format!("{}{}", folders[i - 1], &element[0]));
        i += 1;
    }
    return folders;
}

#[cfg(test)]
mod test {
    use crate::core::write::collect_folder;
    #[test]
    fn regex_check() {
        println!(
            "{:#?}",
            collect_folder("../files/folder.d/folder.mr.d/file.txt")
        );
        println!(
            "{:#?}",
            collect_folder("/files/folder.d/folder.mr.d/file.txt")
        );
        println!(
            "{:#?}",
            collect_folder("./files/folder.d/folder.mr.d/file.txt")
        )
    }
    
}

pub fn file_write(path: &str, text: &str, flag: Flag) -> Result<(), io::Error> {
    match flag {
        Flag::Auto => match get_file(path) {
            Ok(_) => return file_write(path, text, Flag::Old),
            Err(_) => return file_write(path, text, Flag::New),
        },
        Flag::New => match File::create(path) {
            Ok(_) => OpenOptions::new()
                .write(true)
                .create(true)
                .open(path)
                .unwrap()
                .write_all(text.as_bytes()),
            Err(err) => return Err(err.kind().into()),
        },
        Flag::Old => OpenOptions::new()
            // .write(true)
            .append(true)
            .open(path)
            .unwrap()
            .write_all(text.as_bytes()),
    }
}
