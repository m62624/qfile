mod dpds_path;
pub use crate::read::file_read;
mod read {

    use super::dpds_path::io::{self, ErrorKind, Read};
    use super::dpds_path::File;

    pub fn file_read(path: &str) -> Result<String, io::Error> {
        let mut text = String::new();
        if let Err(err) = (match File::open(path) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => return Err(err.kind().into()),
                ErrorKind::PermissionDenied => return Err(err.kind().into()),
                _ => return Err(err.into()),
            },
        })
        .read_to_string(&mut text)
        {
            return Err(err.into());
        }
        Ok(text)
    }
}

mod write {
    use crate::file_read;

    use super::dpds_path::io::{self, ErrorKind, Write};
    use super::dpds_path::File;

    enum Flag {
        New,
        Auto,
        Old,
    }
    // fn file_write(path: &str,text:&str, flag: Flag)->Result<(),io::Error>{
    //     match flag{
    //         Flag::New=> match File::create(path) {
    //             Ok(file)=>file,
    //             Err(err)=>return  Err(err.kind().into()),
                
    //         }.write_all(text.as_bytes()),
    //         Flag::Auto=>match file_read(path).unwrap(){
    //             Ok(x)=>x.
    //         }
    //         },
    //         Flag::Old=>,
    //     }
    // }
}
