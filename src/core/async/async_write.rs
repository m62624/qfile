use super::{AsyncFS, AsyncPath, Error, Flag, QFilePath};
use crate::core::r#async::async_trait::QFileAsync;
use async_recursion::async_recursion;
use async_std::io::WriteExt;
#[async_recursion]
pub async fn async_auto_write<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
    slf: &mut QFilePath,
    text: T,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    //=======================================================
    let sl = slf.context.get_async_pack();
    //=======================================================
    if sl.update_path {
        if cfg!(unix) {
            slf.context.get_async_pack_mut().correct_path = AsyncPath::PathBuf::from(format!(
                "{}{}",
                sl.user_path.to_str().unwrap(),
                sl.file_name.to_str().unwrap()
            ))
        }
        //=======================================================
        let sl = slf.context.get_async_pack();
        //=======================================================
        if cfg!(windows) {
            slf.context.get_async_pack_mut().correct_path = AsyncPath::PathBuf::from(format!(
                "{}{}",
                sl.user_path.to_str().unwrap(),
                sl.file_name.to_str().unwrap()
            ))
        }
    }
    //=======================================================
    let sl = slf.context.get_async_pack();
    //=======================================================
    match sl.flag {
        crate::core::Flag::Old => {
            let async_temp = QFilePath::async_get_path_buf(slf).await?;
            slf.context.get_async_pack_mut().flag = Flag::Auto;
            let mut async_temp = AsyncFS::OpenOptions::new()
                .append(true)
                .open(async_temp)
                .await
                .unwrap();
            let async_temp = async_temp.write_all(text.as_ref().as_bytes());
            async_temp.await.unwrap();
        }
        Flag::New => {
            let async_path = QFilePath::async_get_path_buf(slf).await?;
            let async_file = AsyncFS::File::create(async_path).await;
            match async_file {
                Ok(_) => {
                    let async_temp = QFilePath::async_get_path_buf(slf).await?;
                    slf.context.get_async_pack_mut().update_path = false;
                    slf.context.get_async_pack_mut().flag = Flag::Auto;
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
            let async_path = QFilePath::async_get_path_buf(slf).await?;
            let async_file: Result<AsyncFS::File, Box<dyn Error + Send + Sync>> =
                QFilePath::async_return_file(&async_path.to_str().unwrap()).await;
            match async_file {
                Ok(_) => {
                    slf.context.get_async_pack_mut().flag = Flag::Old;
                    async_auto_write(slf, text).await?;
                }
                Err(err) => {
                    if let Ok(err) = err.downcast::<async_std::io::Error>() {
                        match err.kind() {
                            _ => {
                                let async_dir = QFilePath::async_dir_create(slf, err.kind());
                                async_dir.await.unwrap();
                                async_auto_write(slf, text).await?;
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
    slf: &mut QFilePath,
    text: T,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    slf.context.get_async_pack_mut().flag = Flag::New;
    if let Err(err) = async_auto_write(slf, &text).await {
        if let Ok(err) = err.downcast::<async_std::io::Error>() {
            match err.kind() {
                _ => {
                    let async_dir = QFilePath::async_dir_create(slf, err.kind());
                    async_dir.await?;

                    async_auto_write(slf, &text).await?;
                }
            }
        }
    }
    Ok(())
}
