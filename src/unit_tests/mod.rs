#[cfg(test)]
mod tests_mem;
use crate::mem::{DataSizeUnit, Memory};
use home::home_dir;
use std::path::PathBuf;
use uuid::Uuid;

pub fn expand_path(input_path: String) -> String {
    if input_path.starts_with('~') {
        home_dir()
            .map(|home| {
                let path = if input_path.len() > 2 {
                    home.join(&input_path[2..])
                } else {
                    home.join(&input_path[1..])
                };
                path.display().to_string()
            })
            .unwrap_or_else(|| input_path)
    } else {
        input_path
    }
}
