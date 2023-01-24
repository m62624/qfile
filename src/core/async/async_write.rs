use super::{AsyncFS, AsyncPack, AsyncPath, CodeStatus, QFilePath};
use crate::{core::Flag, AsyncQPackTrait, QPackError};
use async_recursion::async_recursion;
use async_std::io::WriteExt;
use std::error::Error;
impl QFilePath {
    #[async_recursion]
    pub async fn auto_write<T: AsRef<str> + std::marker::Send>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error>> {
        if self.Context.get_async_pack().await.update_path {
            if cfg!(unix) {
                self.Context.get_async_pack_mut().await.correct_path =
                    AsyncPath::PathBuf::from(format!(
                        "{}{}",
                        self.Context
                            .get_async_pack()
                            .await
                            .user_path
                            .to_str()
                            .unwrap(),
                        self.Context
                            .get_async_pack()
                            .await
                            .file_name
                            .to_str()
                            .unwrap()
                    ))
            }
            if cfg!(windows) {
                self.Context.get_async_pack_mut().await.correct_path =
                    AsyncPath::PathBuf::from(format!(
                        "{}{}",
                        self.Context
                            .get_async_pack()
                            .await
                            .user_path
                            .to_str()
                            .unwrap(),
                        self.Context
                            .get_async_pack()
                            .await
                            .file_name
                            .to_str()
                            .unwrap()
                    ))
            }
        }
        match self.Context.get_async_pack().await.flag {
            crate::core::Flag::Old => {
                // ASYNC AWAIT SEND
                let temp = QFilePath::async_get_path_buf(self).await?;
                self.Context.get_async_pack_mut().await.flag = Flag::Auto;
                AsyncFS::OpenOptions::new()
                    .append(true)
                    .open(temp)
                    .await
                    .unwrap()
                    .write_all(text.as_ref().as_bytes());
            }
            Flag::Auto => todo!(),
            Flag::New => todo!(),
        }
        Ok(())
    }
}
/*
pub fn auto_write(&mut self, text: &str) -> Result<(), io::Error> {
        if self.update_path {
            if cfg!(unix) {
                self.correct_path = PathBuf::from(format!(
                    "{}{}",
                    self.user_path.to_str().unwrap(),
                    self.file_name.to_str().unwrap()
                ))
            }
            if cfg!(windows) {
                self.correct_path = PathBuf::from(format!(
                    "{}{}",
                    self.user_path.to_str().unwrap(),
                    self.file_name.to_str().unwrap()
                ))
            }
        }
        match self.flag {
            Flag::Auto => match return_file(self.get_path_buf().to_str().unwrap()) {
                Ok(_) => {
                    self.flag = Flag::Old;
                    return self.auto_write(text);
                }
                Err(err) => match err.kind() {
                    _ => {
                        self.dir_create(err.kind()).unwrap();
                        return self.auto_write(text);
                    }
                },
            },

            Flag::New => match File::create(self.get_path_buf()) {
                Ok(_) => {
                    self.update_path = false;
                    self.flag = Flag::Auto;
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .open(self.get_path_buf())
                        .unwrap()
                        .write_all(text.as_bytes())
                }
                Err(err) => return Err(err),
            },
            Flag::Old => {
                self.flag = Flag::Auto;
                OpenOptions::new()
                    .append(true)
                    .open(self.get_path_buf())
                    .unwrap()
                    .write_all(text.as_bytes())
            }
        }
    }
    fn dir_create(&mut self, err: ErrorKind) -> Result<(), std::io::Error> {
        match err {
            ErrorKind::NotFound => {
                let fullpath = self.get_path_buf().clone();
                let filename = fullpath.file_name().unwrap().to_str().unwrap();
                let path_without_file = fullpath.to_str().unwrap().rsplit_once(filename).unwrap().0;
                {
                    self.user_path = PathBuf::from(path_without_file);
                    self.update_path = true;
                    self.file_name = PathBuf::from(filename);
                    self.flag = Flag::New;
                }
                DirBuilder::new()
                    .recursive(true)
                    .create(path_without_file)
                    .unwrap();
                Ok(())
            }
            _ => Err(err.into()),
        }
    } */
