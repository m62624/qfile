use super::{
    super::{async_trait_crate::async_trait, AsyncPath, QFilePath},
    add_path_for_async, async_directory_create, async_get_path_buf,
    async_read::async_read,
    async_write::{async_auto_write, async_write_only_new},
    Arc, Error,
};
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
    #[allow(unused_variables)]
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
}
