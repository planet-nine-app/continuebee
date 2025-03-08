use async_trait::async_trait;
use axum::http::Uri;
use tokio::io::AsyncWriteExt;

use super::StorageClient;



pub struct FileStorageClient {
    pub storage_uri: Uri,    
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
        tokio::fs::write(self.file_path(key), serde_json::to_string(&value).expect("Failed to serialize value")).await.expect("Failed to write to file");
        Ok(())
    }

    pub async fn write_new(&self, key: &str, value: serde_json::Value) -> anyhow::Result<()> {
        self.create_storage_dir().await.expect("Failed to create storage directory");
        let mut file = match tokio::fs::File::create_new(self.file_path(key)).await {
            Ok(file) => file,
            Err(e) => return Err(e.into()),
        };

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

    async fn set(&self, key: &str, value: serde_json::Value, create: Option<bool>) -> anyhow::Result<()> {
        // create file with key
        match create {
            Some(true) => {
                return self.write_new(key, value).await;
            },
            _ => {
                return self.write(key, value).await;
            }
        }
    }

    async fn delete(&self, key: &str) -> bool {
        tokio::fs::remove_file(self.file_path(key)).await.is_ok()
    }
}

#[cfg(test)]
mod tests {
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
        let dir_path = format!("{}/tmp", current_directory.display());
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
        tokio::fs::remove_dir(dir_path.clone()).await.expect("Failed to remove directory");
    }

    #[tokio::test]
    async fn test_write_new() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/tmp", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        client.write_new(key, value.clone()).await.expect("Failed to write new file");

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, value);

        // attempt to write new again and it should fail
        let result = client.write_new(key, value.clone()).await;
        assert!(result.is_err());

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }

    #[tokio::test]
    async fn test_write_already_existing() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/tmp", current_directory.display());
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
        let file_exists = tokio::fs::metadata(client.file_path(key)).await.is_ok();
        assert!(file_exists);

        // now call write as it alraedy should exist
        // write different value as it should overwrite
        let new_value = serde_json::json!({"new": "value"});
        client.write(key, new_value.clone()).await.expect("Failed to write file");

        let data = tokio::fs::read(client.file_path(key)).await.expect("Failed to read file");
        let data = String::from_utf8(data).expect("Data is not valid UTF-8");
        let data: serde_json::Value = serde_json::from_str(&data).expect("Failed to deserialize data");

        assert_eq!(data, new_value);

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }

    #[tokio::test]
    async fn test_get_data() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/tmp", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let client = FileStorageClient::new(uri);

        let key = "test";
        let value = serde_json::json!({"j": "value"});

        // write to file test with fs::write
        match tokio::fs::write(client.file_path(key), serde_json::to_string(&value).expect("Failed to serialize value")).await {
            Ok(_) => {},
            Err(e) => panic!("Failed to write file: {}", e),
        }

        let data = client.get(key).await.expect("Failed to get data");
        assert_eq!(data, value);

        // clean up
        //tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }
}