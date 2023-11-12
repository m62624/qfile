use super::*;
use sysinfo::{Disk, DiskExt, System, SystemExt};
mod DataSizeUnit_tests {
    use super::*;

    /// Check if the memory size is in bytes.
    #[test]
    fn data_size_unit_t_0() {
        let bytes = 1.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Bytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in kilobytes.
    #[test]
    fn data_size_unit_t_1() {
        let bytes = 1024.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Kilobytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in megabytes.
    #[test]
    fn data_size_unit_t_2() {
        let bytes = 1048576.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Megabytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in gigabytes.
    #[test]
    fn data_size_unit_t_3() {
        let bytes = 1073741824.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Gigabytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in terabytes.
    #[test]
    fn data_size_unit_t_4() {
        let bytes = 1099511627776.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Terabytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in petabytes.
    #[test]
    fn data_size_unit_t_5() {
        let bytes = 1125899906842624.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Petabytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in exabytes.
    #[test]
    fn data_size_unit_t_6() {
        let bytes = 1152921504606846976.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Exabytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }

    /// Check if the memory size is in bytes.
    #[test]
    fn data_size_unit_t_7() {
        let bytes = 1.0;
        let data_size_unit = DataSizeUnit::into_human_readable(bytes);
        match data_size_unit {
            DataSizeUnit::Bytes(_, _) => assert!(true),
            _ => assert!(false),
        }
    }
}

mod Memory_tests {
    use super::*;

    #[test]
    fn get_rom_t_0() {
        assert!(Memory::new(expand_path(String::from("~")))
            .get_rom()
            .is_some());
    }

    #[test]
    fn get_rom_total_t_0() {
        if let Some(rom) = Memory::new(expand_path(String::from("~"))).get_rom() {
            let bytes = match rom.get_total() {
                DataSizeUnit::Bytes(_, b) => b,
                DataSizeUnit::Kilobytes(_, b) => b,
                DataSizeUnit::Megabytes(_, b) => b,
                DataSizeUnit::Gigabytes(_, b) => b,
                DataSizeUnit::Terabytes(_, b) => b,
                DataSizeUnit::Petabytes(_, b) => b,
                DataSizeUnit::Exabytes(_, b) => b,
            };
            assert!(*bytes > 0.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn get_rom_free_t_0() {
        if let Some(rom) = Memory::new(expand_path(String::from("~"))).get_rom() {
            let bytes = match rom.get_free() {
                DataSizeUnit::Bytes(_, b) => b,
                DataSizeUnit::Kilobytes(_, b) => b,
                DataSizeUnit::Megabytes(_, b) => b,
                DataSizeUnit::Gigabytes(_, b) => b,
                DataSizeUnit::Terabytes(_, b) => b,
                DataSizeUnit::Petabytes(_, b) => b,
                DataSizeUnit::Exabytes(_, b) => b,
            };
            assert!(*bytes > 0.0);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn get_ram_available_t_0() {
        let mem = Memory::new(expand_path(String::from("~")));
        let bytes = match mem.get_ram_available() {
            DataSizeUnit::Bytes(_, b) => b,
            DataSizeUnit::Kilobytes(_, b) => b,
            DataSizeUnit::Megabytes(_, b) => b,
            DataSizeUnit::Gigabytes(_, b) => b,
            DataSizeUnit::Terabytes(_, b) => b,
            DataSizeUnit::Petabytes(_, b) => b,
            DataSizeUnit::Exabytes(_, b) => b,
        };
        assert!(*bytes > 0.0);
    }

    #[test]
    fn get_path_t_0() {
        let mem = Memory::new(expand_path(String::from("~")));
        assert!(mem.get_path().len() > 0);
    }

    #[test]
    fn set_path_t_0() {
        let new_path = expand_path(format!("{}/{}", "~", Uuid::new_v4().to_string()));
        let mut mem = Memory::new(expand_path(String::from("~")));
        mem.set_path(expand_path(new_path.clone()));
        let correct_path: &String = mem.get_path();
        assert_eq!(correct_path, &new_path);
    }

    
}
