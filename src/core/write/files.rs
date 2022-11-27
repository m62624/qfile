use crate::dpds_path::fs;
use crate::dpds_path::Path;
use crate::dpds_path::{io, Regex};
use crate::file_read;

pub fn collect_folder(path: &str) -> Vec<String> {
    let mut folders: Vec<String> = Vec::new();
    let mut i = 1;
    let linux = Regex::new(r"^(?:\.\./|\./|[\./]?)|(?:(?:\.\./|\./|[\./])?[^/]*)").unwrap();
    let rgx = linux;
    let mut captures = rgx.captures_iter(path);
    folders.push(captures.next().unwrap()[0].to_string());
    for element in captures {
        folders.push(format!("{}{}", folders[i - 1], &element[0]));
        i += 1;
    }
    return folders;
}

fn files(path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if let Ok(paths) = fs::read_dir(path) {
        for items in paths {
            if let Ok(items) = items {
                files.push(items.path().display().to_string());
            }
        }
    }
    return files;
}
#[test]
fn correct_path_check() {
    // println!("{:#?}", correct_path("./FileS/ToolChain/temp.TXT_old"));
    println!(
        "result: {:#?}",
        file_read(&correct_path("./FileS/ToolChain/temp.TXT_wold").unwrap())
    );
}
#[test]
fn collect_folder_check() {
    println!("{:#?}", collect_folder("/Files/ToolChain/new.file"));
}
pub fn correct_path(path: &str) -> Result<String, io::Error> {
    let mut user_paths = collect_folder(path);

    for i in 0..user_paths.len() {
        let mut really_paths = files(&user_paths[i]);
        println!(
            "user path: {:#?}\nreally_paths: {:#?}",
            user_paths[i], really_paths
        );
        for j in 0..really_paths.len() {
            if user_paths
                .get(i + 1)
                .unwrap_or(&user_paths.get(i).unwrap())
                .to_lowercase()
                == really_paths[j].to_lowercase()
            {
                // println!(
                //     "Совпадение: {} {}",
                //     user_paths[i + 1].to_lowercase(),
                //     really_paths[j].to_lowercase()
                // );
                user_paths[i + 1] = really_paths.remove(j);
                // println!(
                //     "result: {:#?},joined: {:#?}",
                //     user_paths,
                //     user_paths.join("")
                // );
                break;
            }
        }
    }
    println!(
        "&user_paths[user_paths.len() - 1]: {}",
        &user_paths[user_paths.len() - 1]
    );
    // if let Err(err) = file_read(&user_paths[user_paths.len() - 1]) {
    //     println!("ERROR ERROR ERROR");
    //     return Err(err.kind().into());
    // }
    // return Ok(user_paths.pop().unwrap());
    match file_read(&user_paths[user_paths.len() - 1]) {
        Ok(_) => Ok(user_paths.pop().unwrap()),
        Err(err) => {
            if let true = Path::new(&user_paths[user_paths.len() - 1]).is_dir() {
                return Ok(user_paths.pop().unwrap());
            }
            Err(err.kind().into())
        }
    }
}
