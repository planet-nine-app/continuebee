use axum::http::Uri;
use sessionless::Sessionless;

use super::{Client, StorageClient, User};



#[derive(Debug, Clone)]
pub struct UserCLient {
    pub client: Client
}

impl UserCLient {
    pub fn new(storage_uri: Uri) -> Self {
        Self { client: Client::new(storage_uri) }
    }

    fn key(uuid: &str) -> String {
        format!("user:{}", uuid)
    }

    pub fn storage_client(self) -> Box<dyn StorageClient> {
        self.client.storage_client()
    }

    pub async fn get_user(self, uuid: &str) -> Option<User> {
        match self.storage_client().get(UserCLient::key(uuid).as_str()).await {
            Some(value) => {
                match serde_json::from_value(value) {
                    Ok(user) => Some(user),
                    Err(_) => None
                }
            },
            None => None
        }
    }

    pub async fn put_user(self, user: &User) -> anyhow::Result<User> {
        let uuid = Sessionless::generate_uuid().to_string();
        let mut user = user.clone();
        user.uuid = uuid;

        if let Ok(value) = serde_json::to_value(user.clone()) {
            match self.storage_client().set(UserCLient::key(&user.uuid).as_str(), value).await {
                Ok(_) => {
                    return Ok(user.clone());
                },
                Err(e) => Err(e.into()),
            }
        } else {
            Err(anyhow::Error::msg("Failed to serialize user"))
        }
    }

    pub async fn update_hash(self, existing_user: &User, new_hash: String) -> anyhow::Result<User> {
        if let Some(mut user) = self.clone().get_user(&existing_user.uuid).await {
            user.hash = new_hash;
            self.clone().put_user(&user).await
        } else {
            Err(anyhow::Error::msg("Failed to retrieve existing user"))
        }
    }

    pub async fn delete_user(self, uuid: &str) -> bool {
        self.storage_client().delete(UserCLient::key(uuid).as_str()).await
    }

    pub async fn save_keys(self, keys: Vec<&str>) -> anyhow::Result<()> {
        if let Ok(value) = serde_json::to_value(keys) {
            self.storage_client().set("keys", value).await?;
            Ok(())
        } else {
            Err(anyhow::Error::msg("Failed to set keys"))
        }
    }

    pub async fn get_keys(self) -> Vec<String> {
        match self.storage_client().get("keys").await {
            Some(value) => {
                match serde_json::from_value(value) {
                    Ok(result) => result,
                    Err(_) => vec![]
                }
            },
            None => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Uri;
    use tokio::io::AsyncWriteExt;

    #[tokio::test]
    async fn test_get_user() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/get_user", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let initial_uuid = "uuid";
        let file_path = format!("{}/user:{}", dir_path, initial_uuid);
        let user_client = UserCLient::new(uri);

        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        // confirm file doesn't exist before
        let file_exists = tokio::fs::metadata(file_path.clone()).await.is_ok();
        assert!(!file_exists);

        let mut user = User::new("pub_key".to_string(), "hash".to_string());
        user.uuid = initial_uuid.to_string();

        let data= serde_json::to_value(user.clone()).expect("Failed to serialize");

        // write user to file with fs::write
        let mut file = match tokio::fs::File::create_new(file_path).await {
            Ok(file) => file,
            Err(e) => panic!("Failed to write to file: {}", e),
        };

        assert!(file.write_all(serde_json::to_string(&data).expect("Failed to serialize to string").as_bytes()).await.is_ok());

        match user_client.clone().get_user(initial_uuid).await {
            Some(result) => assert_eq!(result, user.clone()),
            None => assert!(false)
        };

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }
}