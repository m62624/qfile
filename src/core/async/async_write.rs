use super::{AsyncFS, AsyncPath, QFilePath};
use crate::{core::Flag, AsyncQPackTrait};
use async_recursion::async_recursion;
use async_std::io::WriteExt;
use std::error::Error;
impl QFilePath {
    /// ASYNC AUTO WRITE
    #[async_recursion]
    pub async fn async_auto_write<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        self: &mut Self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        //=======================================================
        let sl = self.Context.get_pack();
        //=======================================================
        if sl.update_path {
            if cfg!(unix) {
                self.Context.get_pack_mut().correct_path = AsyncPath::PathBuf::from(format!(
                    "{}{}",
                    sl.user_path.to_str().unwrap(),
                    sl.file_name.to_str().unwrap()
                ))
            }
            //=======================================================
            let sl = self.Context.get_pack();
            //=======================================================
            if cfg!(windows) {
                self.Context.get_pack_mut().correct_path = AsyncPath::PathBuf::from(format!(
                    "{}{}",
                    sl.user_path.to_str().unwrap(),
                    sl.file_name.to_str().unwrap()
                ))
            }
        }
        //=======================================================
        let sl = self.Context.get_pack();
        //=======================================================
        match sl.flag {
            crate::core::Flag::Old => {
                let async_temp = QFilePath::async_get_path_buf(self).await?;
                self.Context.get_pack_mut().flag = Flag::Auto;
                let mut async_temp = AsyncFS::OpenOptions::new()
                    .append(true)
                    .open(async_temp)
                    .await
                    .unwrap();
                let async_temp = async_temp.write_all(text.as_ref().as_bytes());
                async_temp.await.unwrap();
            }
            Flag::New => {
                let async_path = QFilePath::async_get_path_buf(self).await?;
                let async_file = AsyncFS::File::create(async_path).await;
                match async_file {
                    Ok(_) => {
                        let async_temp = QFilePath::async_get_path_buf(self).await?;
                        self.Context.get_pack_mut().update_path = false;
                        self.Context.get_pack_mut().flag = Flag::Auto;
                        let mut async_temp = AsyncFS::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .open(async_temp)
                            .await?;
                        let async_temp = async_temp.write_all(text.as_ref().as_bytes());
                        async_temp.await.unwrap();
                    }
                    Err(err) => {
                        return Err(Box::new(err) as Box<dyn Error + Send + Sync>);
                    }
                };
            }
            Flag::Auto => {
                let async_path = QFilePath::async_get_path_buf(self).await?;
                let async_file: Result<AsyncFS::File, Box<dyn Error + Send + Sync>> =
                    QFilePath::async_return_file(&async_path.to_str().unwrap()).await;
                match async_file {
                    Ok(_) => {
                        self.Context.get_pack_mut().flag = Flag::Old;
                        QFilePath::async_auto_write(self, text).await?;
                    }
                    Err(err) => {
                        if let Ok(err) = err.downcast::<async_std::io::Error>() {
                            match err.kind() {
                                _ => {
                                    let async_dir = QFilePath::async_dir_create(self, err.kind());
                                    async_dir.await.unwrap();
                                    self.async_auto_write(text).await?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    #[async_recursion]
    pub async fn async_write_only_new<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.Context.get_pack_mut().flag = Flag::New;
        if let Err(err) = self.async_auto_write(&text).await {
            if let Ok(err) = err.downcast::<async_std::io::Error>() {
                match err.kind() {
                    _ => {
                        let async_dir = QFilePath::async_dir_create(self, err.kind());
                        async_dir.await?;

                        self.async_auto_write(&text).await?;
                    }
                }
            }
        }
        Ok(())
    }
}
