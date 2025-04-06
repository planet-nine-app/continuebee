use async_trait::async_trait;
use axum::http::Uri;
use tokio::{fs::File, io::AsyncWriteExt};

use super::StorageClient;



#[derive(Debug, Clone)]
pub struct FileStorageClient {
    storage_uri: Uri,    
}

impl FileStorageClient {
    pub fn new(storage_uri: Uri) -> Self {
        Self { storage_uri }
    }

    pub fn dir(&self) -> String {
        match self.storage_uri.path().is_empty() {
            true => format!("/{}", self.storage_uri),
            false => self.storage_uri.path().to_string(),
        }
    }

    pub fn file_path(&self, key: &str) -> String {
        // storage_uri is the directory, key is the file name
        format!("{}/{}", self.dir(), key)
    }

    pub async fn create_storage_dir(&self) -> anyhow::Result<bool> {
        // Create the directory if it doesn't exist
        // returns true if the directory was created
        match tokio::fs::create_dir(self.dir()).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    Ok(false)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn write(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()> {
        self.create_storage_dir().await.expect("Failed to create storage directory");

        let file = match tokio::fs::File::create(self.file_path(key)).await {
            Ok(file) => file,
            Err(e) => return Err(e.into()),
        };

        self.serialize_and_write(value, file).await
    }

    pub async fn serialize_and_write(&self, value: serde_json::Value, mut file: File) -> anyhow::Result<()> {
        let serialized = serde_json::to_string(&value).expect("Failed to serialize value");

        match file.write_all(serialized.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

#[async_trait]
impl StorageClient for FileStorageClient {
    async fn get(&self, key: &str) -> Option<serde_json::Value> {

        // Read file to string -> serialize to V
        match tokio::fs::read(self.file_path(key)).await {
            Ok(data) => {
                let data = String::from_utf8(data).expect("Data is not valid UTF-8");
                // deserialize the data to V
                match serde_json::from_str(&data) {
                    Ok(value) => Some(value),
                    Err(_) => None,
                }
            }
            Err(_) => None, 
        }
    }

    async fn set(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()> {
        return self.write(key, value).await
    }

    async fn delete(&self, key: &str) -> bool {
        tokio::fs::remove_file(self.file_path(key)).await.is_ok()
    }
}

#[cfg(test)]
mod tests {
    use crate::test_common::{check_path_exists, cleanup_test_files};

    use super::*;
    use axum::http::Uri;

    #[test]
    fn test_file_path() {
        let file_name = "test";
        let expected_path = "/tmp/test";
        let uri = Uri::from_static("/tmp");
        let client = FileStorageClient::new(uri);
        assert_eq!(client.file_path(file_name), expected_path);

        let uri = Uri::from_static("tmp");
        let client = FileStorageClient::new(uri);
        assert_eq!(client.file_path(file_name), expected_path);
    }

    #[tokio::test]
    async fn test_create_storage_dir() {
        // get project root
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/create_storage_dir", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);
        let created = client.create_storage_dir().await.expect("Failed to create storage directory");

        assert!(created);

        // try to recreate the directory
        let created = client.create_storage_dir().await.expect("Failed to create storage directory");
        // false as it's already there
        assert!(!created);

        // TODO: try to create a directory that we don't have permission to create

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_write_new() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/write_new", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        // confirm file doesn't exist before
        assert!(!check_path_exists(&client.file_path(key)).await);

        client.write(key, value.clone()).await.expect("Failed to write new file");

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, value);

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_write_already_existing() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/write_already_existing", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        // create directory
        client.create_storage_dir().await.expect("Failed to create storage directory");

        // write to file test with fs::write
        let mut file = match tokio::fs::File::create_new(client.file_path(key)).await {
            Ok(file) => file,
            Err(e) => panic!("Failed to write file: {}", e),
        };

        assert!(file.write_all(serde_json::to_string(&value).expect("Failed to serialize").as_bytes()).await.is_ok());
        
        // confirm that the file exists
        assert!(check_path_exists(&client.file_path(key)).await);

        // now call write as it alraedy should exist
        // write different value as it should overwrite
        let new_value = serde_json::json!({"new": "value"});
        client.write(key, new_value.clone()).await.expect("Failed to write file");

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, new_value);

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_set_file_doesnt_exist() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/set_file_doesnt_exist", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        // confirm that the file doesn't exists
        assert!(!check_path_exists(&client.file_path(key)).await);

        let result = client.set(key, value.clone()).await;
        assert!(result.is_ok());

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, value);


        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_set_overwrite() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/set_overwrite", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        // create directory
        client.create_storage_dir().await.expect("Failed to create storage directory");

        // write to file test with fs::write
        let mut file = match tokio::fs::File::create_new(client.file_path(key)).await {
            Ok(file) => file,
            Err(e) => panic!("Failed to write file: {}", e),
        };

        assert!(file.write_all(serde_json::to_string(&value).expect("Failed to serialize").as_bytes()).await.is_ok());
        
        // confirm that the file exists
        assert!(check_path_exists(&client.file_path(key)).await);

        // now call write as it alraedy should exist
        // write different value as it should overwrite
        let new_value = serde_json::json!({"new": "value"});
        client.set(key, new_value.clone()).await.expect("Failed to write file");

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, new_value);

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_get_data() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/get_data", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        client.create_storage_dir().await.expect("Failed to create storage directory");

        // No data at first so should be none
        let result = client.get(key).await;
        assert!(result.is_none());

        // write to file test with fs::write
        match tokio::fs::write(client.file_path(key), serde_json::to_string(&value).expect("Failed to serialize value")).await {
            Ok(_) => {},
            Err(e) => panic!("Failed to write file: {}", e),
        }

        let data = client.get(key).await.expect("Failed to get data");
        assert_eq!(data, value);

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_set_and_get() {

        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/set_and_get", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        client.set(key, value.clone()).await.expect("Failed to set value");

        match client.get(key).await {
            Some(v) => assert_eq!(v, value.clone()),
            None => assert!(false)
        };

        // clean up
        cleanup_test_files(&dir_path).await;
    }

    #[tokio::test]
    async fn test_delete() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/delete", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        client.create_storage_dir().await.expect("Failed to create storage directory");

        // write to file test with fs::write
        match tokio::fs::write(client.file_path(key), serde_json::to_string(&value).expect("Failed to serialize value")).await {
            Ok(_) => {},
            Err(e) => panic!("Failed to write file: {}", e),
        }

        // confirm that the file exists
        assert!(check_path_exists(&client.file_path(key)).await);

        // delete
        assert!(client.delete(key).await);

        // file shouldn't exist
        assert!(!check_path_exists(&client.file_path(key)).await);

        // delete: should be false
        assert!(!client.delete(key).await);

        // clean up
        cleanup_test_files(&dir_path).await;
    }

}