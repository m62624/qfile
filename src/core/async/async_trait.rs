use super::{
    super::{async_trait_crate::async_trait, AsyncPath, QFilePath},
    async_write::{async_auto_write, async_write_only_new},
    Arc, Error, *,
};
use crate::core::r#async::async_read::async_read;
pub use async_std::sync::Mutex as AsyncMutex;

#[async_trait]
pub trait QFileAsync {
    fn add_path_for_async<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        path_file: T,
    ) -> Result<Arc<AsyncMutex<Self>>, Box<dyn Error + Send + Sync>>;
    async fn async_get_path_buf(
        self: &mut Self,
    ) -> Result<AsyncPath::PathBuf, Box<dyn Error + Send + Sync>>;
    async fn async_get_path_string(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(self
            .async_get_path_buf()
            .await?
            .to_str()
            .unwrap()
            .to_owned())
    }
    async fn async_change_path<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        self: &mut Self,
        path: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_read(&mut self) -> Result<String, Box<dyn Error + Send + Sync>>;
    async fn async_auto_write<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_write_only_new<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_directory_create(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_get_file(&mut self) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>>;
}

#[async_trait]
impl QFileAsync for QFilePath {
    async fn async_directory_create(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(async_directory_create(self).await?)
    }
    async fn async_read(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        Ok(async_read(self).await?)
    }
    async fn async_auto_write<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(async_auto_write(self, text).await?)
    }
    async fn async_write_only_new<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(async_write_only_new(self, text).await?)
    }
    fn add_path_for_async<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        path_file: T,
    ) -> Result<Arc<AsyncMutex<Self>>, Box<dyn Error + Send + Sync>> {
        Ok(add_path_for_async(path_file)?)
    }
    async fn async_change_path<T: AsRef<str> + std::marker::Send + std::marker::Sync>(
        self: &mut Self,
        path: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok(self.context.get_async_pack_mut().user_path = AsyncPath::PathBuf::from(path.as_ref()))
    }
    async fn async_get_path_buf(
        self: &mut Self,
    ) -> Result<AsyncPath::PathBuf, Box<dyn Error + Send + Sync>> {
        Ok(async_get_path_buf(self).await?)
    }
    async fn async_get_file(&mut self) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>> {
        Ok(async_get_file(self).await?)
    }
}
