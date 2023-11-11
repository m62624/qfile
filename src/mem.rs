use sysinfo::RefreshKind;

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
#[derive(Debug, Clone, Copy)]
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

/// A structure that stores information about the free and total space on the disk
#[derive(Debug)]
pub struct Rom {
    /// Total space on the disk
    pub total: DataSizeUnit,
    /// Free space on the disk
    pub free: DataSizeUnit,
}

/// A structure that stores the information needed to determine the `optimal' chunk size
#[derive(Debug)]
pub struct Memory<STR: AsRef<str>> {
    /// Here we store an object that can store various data about the system
    system_info: System,
    /// Path to the backup location
    pub path: STR,
    /// Information about the free and total space on the disk.
    /// The device on which the memory is defined depends on the path specified by the user
    pub rom: Option<Rom>,
    /// Information about the free and total space in RAM.
    pub ram_available: DataSizeUnit,
}

impl<STR: AsRef<str>> Memory<STR> {
    /// Creates a new instance of the structure
    pub fn new(backup_location_path: STR) -> Self {
        // only RAM and ROM tracking
        let system_info =
            System::new_with_specifics(RefreshKind::new().with_memory().with_disks_list());
        Self {
            // If the path is not valid, we will not track ROMs
            rom: system_info
                // get all connected disks
                .disks()
                .iter()
                // get each disk's mount point, and compare whether the start
                // of the path matches the path specified by the user.
                .filter_map(|disk| {
                    let temp = disk.mount_point().display().to_string();
                    backup_location_path
                        .as_ref()
                        .starts_with(&temp)
                        .then(|| (disk, temp))
                })
                // problem can occur when one path is a subset of another path
                // -> `/home/user/file.txt` -- specified path
                // -> `/` -- diskA/partitionA
                // -> `/home` -- diskB/partitionB
                // then after checking for the substring, discarding unnecessary mount points,
                // compare the longest possible match.
                .max_by_key(|(_, p)| p.len())
                .map(|(disk, _)| Rom {
                    total: DataSizeUnit::into_human_readable(disk.total_space() as f64),
                    free: DataSizeUnit::into_human_readable(disk.available_space() as f64),
                }),
            ram_available: DataSizeUnit::into_human_readable(system_info.available_memory() as f64),
            path: backup_location_path,
            system_info,
        }
    }

    pub fn update_info(&mut self) {
        self.system_info.refresh_memory();
        self.system_info.refresh_disks_list();
    }
}

#[test]
fn test_memory() {
    use std::thread::sleep;
    use std::time::Duration;
    let mut memory = Memory::new("/");
    println!("{:#?}", memory);
    sleep(Duration::from_secs(30));
    memory.update_info();
    println!("{:#?}", memory);
}
