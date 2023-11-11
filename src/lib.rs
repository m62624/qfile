mod mem;
#[cfg(test)]
mod unit_tests;

use import_libs::*;

pub use mem::{MeasurementUnit, MemInfo};

mod import_libs {
    pub use home::home_dir;
    pub use std::fs;
    pub use std::fs::File;
    pub use std::io::{BufReader, Read, Result, Write};
    pub use std::path::PathBuf;
    pub use std::time::Instant;
    pub use sys_info::{disk_info, mem_info};
    pub use unicase::UniCase;
    pub use uuid::Uuid;
}

#[derive(Debug, Clone, PartialEq)]
pub enum CodeStatus {
    SyncStatus,
    AsyncStatus,
}

#[derive(Debug, Clone)]
pub struct QFile {
    path: UniCase<PathBuf>,
    case_sensitive: bool,
    code_status: CodeStatus,
}

#[derive(Debug)]
pub struct IterQfile {
    file: BufReader<File>,
    chunk_size: usize,
}

impl IterQfile {
    pub fn new(file: File, chunk_size: usize) -> Result<Self> {
        let buf_reader = BufReader::new(file);
        Ok(IterQfile {
            file: buf_reader,
            chunk_size,
        })
    }

    fn calculate_optimal_chunk_size(file_size: u128) -> Option<usize> {
        const MIN_PERCENTAGE: f64 = 0.15;

        let max_chunk_size = (file_size as f64 * (1.0 - MIN_PERCENTAGE)).ceil() as usize;

        for chunk_size in (1..=max_chunk_size).rev() {
            if file_size % chunk_size as u128 == 0 {
                return Some(chunk_size);
            }
        }

        None
    }
}

#[test]
fn x() {
    let iter_file = IterQfile::calculate_optimal_chunk_size(6291456u128);
    if let Some(chunk_size) = iter_file {
        println!("{:#?}", MeasurementUnit::from_bytes_per_second(chunk_size as f64));
    }
}

impl Iterator for IterQfile {
    type Item = Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = vec![0; self.chunk_size];
        match self.file.read(&mut chunk) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Если считано меньше байт, чем размер чанка, обрезаем вектор до актуального размера
                chunk.truncate(bytes_read);
                Some(Ok(chunk))
            }
            Ok(_) => None,              // Достигнут конец файла
            Err(err) => Some(Err(err)), // Ошибка чтения
        }
    }
}
