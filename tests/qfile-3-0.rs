use async_std::task;
use qfile::{async_trait::*, sync_trait::*, QFilePath, QPackError};
use rand::Rng;
use std::{error::Error, fs, iter};
pub struct TestFolder {
    root_folder: String,
    path: String,
}
enum OSoption {
    Windows,
    Unix,
    None,
}
impl Default for OSoption {
    fn default() -> Self {
        if cfg!(windows) {
            return OSoption::Windows;
        }
        if cfg!(unix) {
            return OSoption::Unix;
        }
        OSoption::None
    }
}
impl TestFolder {
    fn new(root_folder: &str, path: &str, os: OSoption) -> Self {
        let generate = |len: usize| -> String {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+-";
            let mut rng = rand::thread_rng();
            let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
            iter::repeat_with(one_char).take(len).collect()
        };
        let root_folder = format!("{}-{}", root_folder, generate(15));
        match os {
            OSoption::Unix => {
                return TestFolder {
                    root_folder: root_folder.to_owned(),
                    path: format!("{}/{}", root_folder, path),
                }
            }
            OSoption::Windows => {
                return TestFolder {
                    root_folder: root_folder.to_owned(),
                    path: format!("{}\\{}", root_folder, path),
                };
            }
            OSoption::None => panic!("{}", QPackError::SystemNotDefined),
        }
    }
    fn delete_temp_folder(&self) {
        if let Ok(_) = fs::remove_dir_all(&self.root_folder) {
            dbg!("removed root_temp");
        }
    }
}

mod r#async {
    use crate::*;
    use std::sync::Arc;
    #[async_std::test]
    #[should_panic(expected = "Synchronous call from AsyncPack")]
    async fn async_sync_error() {
        QFilePath::add_path_for_async(
            TestFolder::new(".Polygon", Default::default(), Default::default()).path,
        )
        .unwrap()
        .lock()
        .await
        .auto_write("")
        .unwrap();
    }
    #[async_std::test]
    async fn constructor_init() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        QFilePath::add_path_for_async(
            TestFolder::new(".Polygon", Default::default(), Default::default()).path,
        )?;
        Ok(())
    }
    #[async_std::test]
    async fn auto_write_single() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        QFilePath::add_path_for_async(&path.path)?
            .lock()
            .await
            .async_auto_write(":D")
            .await?;
        assert_eq!(async_std::path::Path::new(&path.path).exists().await, true);
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn auto_write_multi() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let file = QFilePath::add_path_for_async(&path.path)?;
        for _ in 0..3 {
            let file_cl = Arc::clone(&file);
            task::block_on(async {
                file_cl.lock().await.async_auto_write(":D").await?;
                Ok::<(), Box<dyn Error + Sync + std::marker::Send>>(())
            })?;
        }
        assert_eq!(file.lock().await.async_read().await?, ":D:D:D");
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn write_only_new_single() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let file = QFilePath::add_path_for_async(&path.path)?;
        file.lock().await.async_write_only_new(":D").await?;
        file.lock().await.async_write_only_new(":D").await?;
        assert_eq!(file.lock().await.async_read().await?, ":D");
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn write_only_new_multi() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let file = QFilePath::add_path_for_async(&path.path)?;
        for _ in 0..3 {
            let file_cl = Arc::clone(&file);
            task::block_on(async {
                file_cl.lock().await.async_write_only_new(":D").await?;
                Ok::<(), Box<dyn Error + Sync + std::marker::Send>>(())
            })?;
        }
        assert_eq!(file.lock().await.async_read().await?, ":D");
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn read_single() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let file = QFilePath::add_path_for_async(&path.path)?;
        file.lock().await.async_auto_write(":D").await?;
        let data = file.lock().await.async_read().await?;
        assert_eq!(data, ":D");
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn read_multi() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let file = QFilePath::add_path_for_async(&path.path)?;
        file.lock().await.async_auto_write(":D :D :D :D :D").await?;
        for _ in 0..3 {
            let file_cl = Arc::clone(&file);
            task::block_on(async {
                assert_eq!(file_cl.lock().await.async_read().await?, ":D :D :D :D :D");
                Ok::<(), Box<dyn Error + Sync + std::marker::Send>>(())
            })?;
        }
        path.delete_temp_folder();
        Ok(())
    }
    #[cfg(test)]
    #[cfg(target_family = "unix")]
    mod linux {
        use super::*;

        #[async_std::test]
        #[should_panic(expected = "UnixPathIsIncorrect")]
        async fn constructor_error() {
            QFilePath::add_path_for_async(
                TestFolder::new(".Polygon", Default::default(), OSoption::Windows).path,
            )
            .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn auto_write_error() {
            QFilePath::add_path_for_async("/file.txt")
                .unwrap()
                .lock()
                .await
                .async_auto_write(":D")
                .await
                .unwrap();
        }
        #[async_std::test]
        async fn auto_write_init_folder_with_other(
        ) -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".Polygon", "FolderX/file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_auto_write("").await?;
            file.lock()
                .await
                .async_change_path(format!(
                    "{}/folderX/FolderY/new_file.txt",
                    path.root_folder.to_lowercase()
                ))
                .await?;
            file.lock().await.async_auto_write("").await?;
            assert_eq!(
                file.lock().await.async_get_path_string().await?,
                format!("./{}/FolderX/FolderY/new_file.txt", &path.root_folder)
            );
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn read_error() {
            QFilePath::add_path_for_async("/etc/shadow")
                .unwrap()
                .lock()
                .await
                .async_read()
                .await
                .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn write_only_new_error() {
            let file = QFilePath::add_path_for_async("/file.txt").unwrap();
            file.lock().await.async_write_only_new(":D").await.unwrap();
        }
        #[async_std::test]
        async fn directory_create() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".Polygon", "x/y/z", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_directory_create().await?;
            assert_eq!(async_std::path::Path::new(&path.path).exists().await, true);
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn directory_create_error() {
            QFilePath::add_path_for_async("/x/y/z")
                .unwrap()
                .lock()
                .await
                .async_directory_create()
                .await
                .unwrap();
        }
        #[async_std::test]
        async fn change_path() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            assert_eq!(
                file.lock().await.async_get_path_string().await?,
                String::from(path.path)
            );
            let path = TestFolder::new("./.Polygon", "new_file.txt", Default::default());
            file.lock().await.async_change_path(&path.path).await?;
            assert_eq!(
                file.lock().await.async_get_path_string().await?,
                String::from(path.path)
            );
            Ok(())
        }
        #[async_std::test]
        async fn get_path_buf() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_auto_write("").await?;
            file.lock().await.async_get_path_buf().await?;
            let _path = format!("{}/FILE.txt", path.root_folder);
            let file = QFilePath::add_path_for_async(&_path)?;
            assert_eq!(
                file.lock().await.async_get_path_buf().await?,
                async_std::path::PathBuf::from(&path.path)
            );
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        async fn get_path_string() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_auto_write("").await?;
            file.lock().await.async_get_path_string().await?;
            let _path = format!("{}/FILE.txt", path.root_folder);
            let file = QFilePath::add_path_for_async(&_path)?;
            assert_eq!(
                &file.lock().await.async_get_path_string().await?,
                &path.path
            );
            path.delete_temp_folder();
            Ok(())
        }
    }
    #[cfg(test)]
    #[cfg(target_family = "windows")]
    mod windows {
        use super::*;

        #[async_std::test]
        #[should_panic(expected = "WindowsPathIsIncorrect")]
        async fn constructor_error() {
            QFilePath::add_path_for_async(
                TestFolder::new(".Polygon", Default::default(), OSoption::Unix).path,
            )
            .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn auto_write_error() {
            QFilePath::add_path_for_async("C:\\Windows\\System32\\file.txt")
                .unwrap()
                .lock()
                .await
                .async_auto_write(":D")
                .await
                .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn read_error() {
            QFilePath::add_path_for_async("C:\\Windows\\System32\\winrm.vbs")
                .unwrap()
                .lock()
                .await
                .async_read()
                .await
                .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn write_only_new_error() {
            let file = QFilePath::add_path_for_async("C:\\Windows\\System32\\file.txt").unwrap();
            file.lock().await.async_write_only_new(":D").await.unwrap();
        }
        #[async_std::test]
        async fn directory_create() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".Polygon", "x\\y\\z", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_directory_create().await?;
            assert_eq!(async_std::path::Path::new(&path.path).exists().await, true);
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn directory_create_error() {
            QFilePath::add_path_for_async("C:\\Windows\\System32\\x\\y\\z")
                .unwrap()
                .lock()
                .await
                .async_directory_create()
                .await
                .unwrap();
        }
        #[async_std::test]
        async fn change_path() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            assert_eq!(
                file.lock().await.async_get_path_string().await?,
                String::from(path.path)
            );
            let path = TestFolder::new(".\\.Polygon", "new_file.txt", Default::default());
            file.lock().await.async_change_path(&path.path).await?;
            assert_eq!(
                file.lock().await.async_get_path_string().await?,
                String::from(path.path)
            );
            Ok(())
        }
        #[async_std::test]
        async fn get_path_buf() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_auto_write("").await?;
            file.lock().await.async_get_path_buf().await?;
            let _path = format!("{}\\FILE.txt", path.root_folder);
            let file = QFilePath::add_path_for_async(&_path)?;
            assert_eq!(
                file.lock().await.async_get_path_buf().await?,
                async_std::path::PathBuf::from(&path.path)
            );
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        async fn get_path_string() -> Result<(), Box<dyn Error + Sync + std::marker::Send>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let file = QFilePath::add_path_for_async(&path.path)?;
            file.lock().await.async_auto_write("").await?;
            file.lock().await.async_get_path_string().await?;
            let _path = format!("{}\\FILE.txt", path.root_folder);
            let file = QFilePath::add_path_for_async(&_path)?;
            assert_eq!(
                &file.lock().await.async_get_path_string().await?,
                &path.path
            );
            path.delete_temp_folder();
            Ok(())
        }
    }
}
mod r#sync {
    use crate::*;
    #[async_std::test]
    #[should_panic(expected = "Asynchronous call from SyncPack")]
    async fn sync_async_error() {
        QFilePath::add_path(
            TestFolder::new(".Polygon", Default::default(), Default::default()).path,
        )
        .unwrap()
        .async_auto_write("")
        .await
        .unwrap();
    }
    #[async_std::test]
    async fn constructor_init() -> Result<(), Box<dyn Error>> {
        QFilePath::add_path(
            TestFolder::new(".Polygon", Default::default(), Default::default()).path,
        )?;
        Ok(())
    }
    #[async_std::test]
    async fn auto_write_single() -> Result<(), Box<dyn Error>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        QFilePath::add_path(&path.path)?.auto_write(":D")?;
        assert_eq!(std::path::Path::new(&path.path).exists(), true);
        path.delete_temp_folder();
        Ok(())
    }

    #[async_std::test]
    async fn write_only_new_single() -> Result<(), Box<dyn Error>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let mut file = QFilePath::add_path(&path.path)?;
        file.write_only_new(":D")?;
        file.write_only_new(":D")?;
        assert_eq!(file.read()?, ":D");
        path.delete_temp_folder();
        Ok(())
    }
    #[async_std::test]
    async fn read_single() -> Result<(), Box<dyn Error>> {
        let path = TestFolder::new(".Polygon", "file.txt", Default::default());
        let mut file = QFilePath::add_path(&path.path)?;
        file.auto_write(":D")?;
        let data = file.read()?;
        assert_eq!(data, ":D");
        path.delete_temp_folder();
        Ok(())
    }
    #[cfg(test)]
    #[cfg(target_family = "unix")]
    mod linux {
        use super::*;
        #[async_std::test]
        #[should_panic(expected = "UnixPathIsIncorrect")]
        async fn constructor_error() {
            QFilePath::add_path(
                TestFolder::new(".Polygon", Default::default(), OSoption::Windows).path,
            )
            .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn auto_write_error() {
            QFilePath::add_path("/file.txt")
                .unwrap()
                .auto_write(":D")
                .unwrap();
        }
        #[async_std::test]
        async fn auto_write_init_folder_with_other() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".Polygon", "FolderX/file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.auto_write("")?;
            file.change_path(format!(
                "{}/folderX/FolderY/new_file.txt",
                path.root_folder.to_lowercase()
            ))?;
            file.auto_write("")?;
            assert_eq!(
                file.get_path_string()?,
                format!("./{}/FolderX/FolderY/new_file.txt", &path.root_folder)
            );
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn read_error() {
            QFilePath::add_path("/etc/shadow").unwrap().read().unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn write_only_new_error() {
            let mut file = QFilePath::add_path("/file.txt").unwrap();
            file.write_only_new(":D").unwrap();
        }
        #[async_std::test]
        async fn directory_create() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".Polygon", "x/y/z", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.directory_create()?;
            assert_eq!(std::path::Path::new(&path.path).exists(), true);
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn directory_create_error() {
            QFilePath::add_path("/x/y/z")
                .unwrap()
                .directory_create()
                .unwrap();
        }
        #[async_std::test]
        async fn change_path() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            assert_eq!(file.get_path_string()?, String::from(path.path));
            let path = TestFolder::new("./.Polygon", "new_file.txt", Default::default());
            file.change_path(&path.path)?;
            assert_eq!(file.get_path_string()?, String::from(path.path));
            Ok(())
        }
        #[async_std::test]
        async fn get_path_buf() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.auto_write("")?;
            file.get_path_buf()?;
            let _path = format!("{}/FILE.txt", path.root_folder);
            let mut file = QFilePath::add_path(&_path)?;
            assert_eq!(file.get_path_buf()?, std::path::PathBuf::from(&path.path));
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        async fn get_path_string() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new("./.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.auto_write("")?;
            file.get_path_string()?;
            let _path = format!("{}/FILE.txt", path.root_folder);
            let mut file = QFilePath::add_path(&_path)?;
            assert_eq!(&file.get_path_string()?, &path.path);
            path.delete_temp_folder();
            Ok(())
        }
    }
    #[cfg(test)]
    #[cfg(target_family = "windows")]
    mod windows {
        use super::*;
        #[async_std::test]
        #[should_panic(expected = "WindowsPathIsIncorrect")]
        async fn constructor_error() {
            QFilePath::add_path(
                TestFolder::new(".Polygon", Default::default(), OSoption::Unix).path,
            )
            .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn auto_write_error() {
            QFilePath::add_path("C:\\Windows\\System32\\file.txt")
                .unwrap()
                .auto_write(":D")
                .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn read_error() {
            QFilePath::add_path("C:\\Windows\\System32\\winrm.vbs")
                .unwrap()
                .read()
                .unwrap();
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn write_only_new_error() {
            let mut file = QFilePath::add_path("C:\\Windows\\System32\\file.txt").unwrap();
            file.write_only_new(":D").unwrap();
        }
        #[async_std::test]
        async fn directory_create() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".Polygon", "x\\y\\z", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.directory_create()?;
            assert_eq!(std::path::Path::new(&path.path).exists(), true);
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        #[should_panic(expected = "PermissionDenied")]
        async fn directory_create_error() {
            QFilePath::add_path("C:\\Windows\\System32\\x\\y\\z")
                .unwrap()
                .directory_create()
                .unwrap();
        }
        #[async_std::test]
        async fn change_path() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            assert_eq!(file.get_path_string()?, String::from(path.path));
            let path = TestFolder::new(".\\.Polygon", "new_file.txt", Default::default());
            file.change_path(&path.path)?;
            assert_eq!(file.get_path_string()?, String::from(path.path));
            Ok(())
        }
        #[async_std::test]
        async fn get_path_buf() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.auto_write("")?;
            file.get_path_buf()?;
            let _path = format!("{}\\FILE.txt", path.root_folder);
            let mut file = QFilePath::add_path(&_path)?;
            assert_eq!(file.get_path_buf()?, std::path::PathBuf::from(&path.path));
            path.delete_temp_folder();
            Ok(())
        }
        #[async_std::test]
        async fn get_path_string() -> Result<(), Box<dyn Error>> {
            let path = TestFolder::new(".\\.Polygon", "file.txt", Default::default());
            let mut file = QFilePath::add_path(&path.path)?;
            file.auto_write("")?;
            file.get_path_string()?;
            let _path = format!("{}\\FILE.txt", path.root_folder);
            let mut file = QFilePath::add_path(&_path)?;
            assert_eq!(&file.get_path_string()?, &path.path);
            path.delete_temp_folder();
            Ok(())
        }
    }
}
