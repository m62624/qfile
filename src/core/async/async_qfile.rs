pub use super::super::Directory;
use super::{
    super::{async_trait_crate::async_trait, *},
    async_find::{async_find_paths, async_find_regex_paths},
    async_write::*,
    Arc, Error, *,
};
use crate::core::r#async::async_read::async_read;
pub use async_std::channel as AsyncChannel;
use async_std::channel::{SendError, Sender};
use async_std::path::PathBuf;
pub use async_std::sync::Mutex as AsyncMutex;
#[async_trait]
pub trait TraitQFileAsync {
    fn add_path_for_async<T: AsRef<str> + Send + Sync>(
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
    async fn async_change_path<T: AsRef<str> + Send + Sync>(
        self: &mut Self,
        path: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_read(&mut self) -> Result<String, Box<dyn Error + Send + Sync>>;
    async fn async_auto_write<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_write_only_new<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_directory_create(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn async_get_file(&mut self) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>>;
    async fn async_find_paths<T: AsRef<str> + Send + Sync + 'static>(
        place: Directory,
        name: T,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>>;
    async fn async_find_regex_paths(
        place: Directory,
        name: Regex,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>>;
}

#[async_trait]
impl TraitQFileAsync for QFilePath {
    async fn async_directory_create(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        async_directory_create(self).await
    }
    async fn async_read(&mut self) -> Result<String, Box<dyn Error + Send + Sync>> {
        async_read(self).await
    }
    async fn async_auto_write<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        async_auto_write(self, text).await
    }
    async fn async_write_only_new<T: AsRef<str> + Send + Sync>(
        &mut self,
        text: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        async_write_only_new(self, text).await
    }
    fn add_path_for_async<T: AsRef<str> + Send + Sync>(
        path_file: T,
    ) -> Result<Arc<AsyncMutex<Self>>, Box<dyn Error + Send + Sync>> {
        add_path_for_async(path_file)
    }
    async fn async_change_path<T: AsRef<str> + Send + Sync>(
        self: &mut Self,
        path: T,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        Ok({
            self.context.get_async_pack_mut().user_path = AsyncPath::PathBuf::from(path.as_ref());
            self.context.get_async_pack_mut().correct_path = Default::default();
            self.context.get_async_pack_mut().request_items.clear();
        })
    }
    async fn async_get_path_buf(
        self: &mut Self,
    ) -> Result<AsyncPath::PathBuf, Box<dyn Error + Send + Sync>> {
        async_get_path_buf(self).await
    }
    async fn async_get_file(&mut self) -> Result<AsyncFS::File, Box<dyn Error + Send + Sync>> {
        async_get_file(self).await
    }
    async fn async_find_paths<T: AsRef<str> + Send + Sync + 'static>(
        place: Directory,
        name: T,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>> {
        async_find_paths(place, name, follow_link, sender).await
    }
    async fn async_find_regex_paths(
        place: Directory,
        name: Regex,
        follow_link: bool,
        sender: Sender<PathBuf>,
    ) -> Result<(), SendError<PathBuf>> {
        async_find_regex_paths(place, name, follow_link, sender).await
    }
}
