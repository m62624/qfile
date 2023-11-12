use super::*;
mod DataSizeUnit_test {
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
