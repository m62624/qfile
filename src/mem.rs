use std::fmt::Display;

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

/// Returns the total amount of memory in bytes (`human_readable`, `bytes`).
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

#[derive(Debug)]
/// A structure that stores the information needed to determine the `optimal' chunk size
pub struct Memory<S: AsRef<str>> {
    /// Here we store an object that can store various data about the system
    system_info: System,
    /// Path to the backup location
    path: S,
    /// Information about the free and total space on the disk.
    /// The device on which the memory is defined depends on the path specified by the user
    rom: Option<Rom>,
    /// Information about the free and total space in RAM.
    ram_available: DataSizeUnit,
}

impl<S: AsRef<str>> Memory<S> {
    /// Creates a new instance of the structure
    pub fn new(path_on_disk: S) -> Self {
        // only RAM and ROM tracking
        let mut system_info =
            System::new_with_specifics(RefreshKind::new().with_memory().with_disks_list());
        Self {
            rom: Self::locate_the_disk_in_the_path(&mut system_info, &path_on_disk, false)
                .map(|disk| Rom {
                    total: DataSizeUnit::into_human_readable(disk.total_space() as f64),
                    free: DataSizeUnit::into_human_readable(disk.available_space() as f64),
                }),
            ram_available: DataSizeUnit::into_human_readable(system_info.available_memory() as f64),
            path: path_on_disk,
            system_info,
        }
    }

    /// Update information about the free and total space on the disk.
    pub fn update_info(&mut self) {
        self.system_info.refresh_memory();
        self.rom = Self::locate_the_disk_in_the_path(&mut self.system_info, &self.path, true)
            .map(|disk| Rom {
                total: DataSizeUnit::into_human_readable(disk.total_space() as f64),
                free: DataSizeUnit::into_human_readable(disk.available_space() as f64),
            });
        self.ram_available =
            DataSizeUnit::into_human_readable(self.system_info.available_memory() as f64);
    }

    /// Get each disk's mount point, and compare whether the start
    /// of the path matches the path specified by the user.
    ///
    /// ---
    /// **Problem can occur when one path is a subset of another path**\
    /// * `/home/user/file.txt` -- specified path\
    /// * `/` -- diskA / partitionA\
    /// * `/home` -- diskB / partitionB\
    /// then after checking for the substring, discarding unnecessary mount points,
    /// compare the longest possible match.
    pub fn locate_the_disk_in_the_path<'a>(
        system_info: &'a mut System,
        path: &S,
        update_system_info: bool,
    ) -> Option<&'a Disk> {
        if update_system_info {
            system_info.refresh_disks_list();
        }
        system_info
            .disks()
            .iter()
            .filter_map(|disk| {
                let temp = disk.mount_point().display().to_string();
                path.as_ref().starts_with(&temp).then(|| (disk, temp))
            })
            .max_by_key(|(_, p)| p.len())
            .map(|(disk, _)| disk)
    }

    pub fn get_rom(&self) -> Option<&Rom> {
        self.rom.as_ref()
    }

    pub fn get_ram_available(&self) -> &DataSizeUnit {
        &self.ram_available
    }

    pub fn get_path(&self) -> &S {
        &self.path
    }

    /// Set the path to the backup location
    /// Automatically updates the information about the free and total space on the disk.
    /// (hidden call `update_info`).
    pub fn set_path(&mut self, path: S) {
        self.path = path;
        self.update_info();
    }
}

impl Display for DataSizeUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataSizeUnit::Bytes(_, bytes) => write!(f, "{} bytes", bytes),
            DataSizeUnit::Kilobytes(kb, _) => write!(f, "{} KB", kb),
            DataSizeUnit::Megabytes(mb, _) => write!(f, "{} MB", mb),
            DataSizeUnit::Gigabytes(gb, _) => write!(f, "{} GB", gb),
            DataSizeUnit::Terabytes(tb, _) => write!(f, "{} TB", tb),
            DataSizeUnit::Petabytes(pb, _) => write!(f, "{} PB", pb),
            DataSizeUnit::Exabytes(eb, _) => write!(f, "{} EB", eb),
        }
    }
}

impl Display for Rom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[total: {}, free: {}]", self.total, self.free,)
    }
}

impl<S: AsRef<str>> Display for Memory<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Free RAM: {}, ROM: {}, Path: {}",
            self.ram_available,
            if let Some(rom) = &self.rom {
                rom.to_string()
            } else {
                "None".into()
            },
            self.path.as_ref()
        )
    }
}
