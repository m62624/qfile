use super::import_libs::*;

/// Kilobytes in bytes.
const BYTES_IN_KB: f64 = 1024.0;
/// Megabytes in bytes.
const BYTES_IN_MB: f64 = BYTES_IN_KB * BYTES_IN_KB;
/// Gigabytes in bytes.
const BYTES_IN_GB: f64 = BYTES_IN_MB * BYTES_IN_KB;
/// Terabytes in bytes.
const BYTES_IN_TB: f64 = BYTES_IN_GB * BYTES_IN_KB;
/// Petabytes in bytes.
const BYTES_IN_PB: f64 = BYTES_IN_TB * BYTES_IN_KB;
/// Exabytes in bytes.
const BYTES_IN_EB: f64 = BYTES_IN_PB * BYTES_IN_KB;

/// Returns the total amount of memory in bytes.
#[derive(Debug)]
pub enum DataSizeUnit {
    Bytes(usize, f64),
    Kilobytes(usize, f64),
    Megabytes(usize, f64),
    Gigabytes(usize, f64),
    Terabytes(usize, f64),
    Petabytes(usize, f64),
    Exabytes(usize, f64),
}

impl DataSizeUnit {
    /// Human readable representation of the memory size. (`human_readable`, `bytes`)
    pub fn into_human_readable(bytes: f64) -> DataSizeUnit {
        if bytes < BYTES_IN_KB {
            DataSizeUnit::Bytes(bytes as usize, bytes)
        } else if bytes < BYTES_IN_MB {
            DataSizeUnit::Kilobytes((bytes / BYTES_IN_KB) as usize, bytes)
        } else if bytes < BYTES_IN_GB {
            DataSizeUnit::Megabytes((bytes / BYTES_IN_MB) as usize, bytes)
        } else if bytes < BYTES_IN_TB {
            DataSizeUnit::Gigabytes((bytes / BYTES_IN_GB) as usize, bytes)
        } else if bytes < BYTES_IN_PB {
            DataSizeUnit::Terabytes((bytes / BYTES_IN_TB) as usize, bytes)
        } else if bytes < BYTES_IN_EB {
            DataSizeUnit::Petabytes((bytes / BYTES_IN_PB) as usize, bytes)
        } else {
            DataSizeUnit::Exabytes((bytes / BYTES_IN_EB) as usize, bytes)
        }
    }
}

#[test]
fn main() {}
