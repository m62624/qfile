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

#[derive(Debug)]
pub struct Rom {
    pub total: DataSizeUnit,
    pub free: DataSizeUnit,
}

#[derive(Debug)]
pub struct Memory<STR: AsRef<str>> {
    system_info: System,
    pub path: STR,
    pub rom: Option<Rom>,
    pub ram_available: DataSizeUnit,
}

impl<STR: AsRef<str>> Memory<STR> {
    pub fn new(backup_location_path: STR) -> Self {
        let system_info =
            System::new_with_specifics(RefreshKind::new().with_memory().with_disks_list());
        Self {
            rom: system_info
                .disks()
                .iter()
                .find(|disk| {
                    println!(
                        "{:#?}, {:#?}",
                        disk.mount_point().to_path_buf().display().to_string(),
                        backup_location_path.as_ref().to_string()
                    );
                    backup_location_path
                        .as_ref()
                        .starts_with(&disk.mount_point().to_path_buf().display().to_string())
                })
                .map(|disk| Rom {
                    total: DataSizeUnit::into_human_readable(disk.total_space() as f64),
                    free: DataSizeUnit::into_human_readable(disk.available_space() as f64),
                }),
            ram_available: DataSizeUnit::into_human_readable(system_info.available_memory() as f64),
            path: backup_location_path,
            system_info,
        }
    }

    pub fn update_info(&mut self, path: Option<STR>) {
        self.system_info.refresh_memory();
        self.system_info.refresh_disks_list();
        self.rom = self
            .system_info
            .disks()
            .iter()
            .find(|disk| {
                disk.mount_point().to_path_buf().display().to_string()
                    == if let Some(path) = path.as_ref() {
                        path.as_ref().to_string()
                    } else {
                        self.path.as_ref().to_string()
                    }
            })
            .map(|disk| Rom {
                total: DataSizeUnit::into_human_readable(disk.total_space() as f64),
                free: DataSizeUnit::into_human_readable(disk.available_space() as f64),
            });
        self.ram_available =
            DataSizeUnit::into_human_readable(self.system_info.available_memory() as f64);
    }
}

#[test]
fn main() {
    let mem_info = Memory::new("/home/m62624/Проекты/flexible_inspect/");
    dbg!(mem_info);
}
