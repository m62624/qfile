use qfile::{file_read, file_write, Flag};
use std::fs::{self};
struct Basic {
    outside: String,
    inside: String,
}
impl Basic {
    fn paths(&self, element: usize) -> String {
        let paths = vec![
            format!("{}{}", self.inside, "/temp1/file.txt"),
            format!("{}{}{}", "./", self.inside, "/temp2/file.txt"),
            format!("{}{}", self.outside, "/file.txt"),
            format!("{}{}", self.outside, "/qfile_temp_temp.txt/file.txt"),
        ];
        return paths.get(element).unwrap().clone();
    }
    fn new() -> Self {
        Basic {
            outside: "../qfile_temp_00_00".to_string(),
            inside: "Polygon".to_string(),
        }
    }
}

#[test]
fn test_0_auto() {
    file_write(&Basic::new().paths(0)[..], "ok", Flag::Auto).unwrap();
    assert_eq!(file_read(&Basic::new().paths(0)[..]).unwrap(), "ok");
}
#[test]
fn test_1_auto() {
    file_write(&Basic::new().paths(1)[..], "ok", Flag::Auto).unwrap();
    assert_eq!(file_read(&Basic::new().paths(1)[..]).unwrap(), "ok");
}
#[test]
fn test_2_auto() {
    if !file_read(&Basic::new().paths(2)[..]).unwrap().is_empty() {
        fs::remove_file(&Basic::new().paths(2)[..]).unwrap();
    }
    file_write(&Basic::new().paths(2)[..], "ok", Flag::Auto).unwrap();
    file_write(&Basic::new().paths(2)[..], "ok", Flag::Auto).unwrap();
    assert_eq!(file_read(&Basic::new().paths(2)[..]).unwrap(), "okok");
}
#[test]
fn test_3_auto() {
    file_write(&Basic::new().paths(3)[..], "ok", Flag::Auto).unwrap();
    assert_eq!(file_read(&Basic::new().paths(3)[..]).unwrap(), "ok");
}

#[test]
#[should_panic]
#[ignore]
fn removeall() {
    let bsc = Basic::new();
    fs::remove_dir_all(bsc.inside).unwrap();
    fs::remove_dir_all(bsc.outside).unwrap()
}
