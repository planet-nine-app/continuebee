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


    #[tokio::test]
    async fn test_put_new_user() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/put_new_user", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let user_client = UserCLient::new(uri);

        // check that dir_path doesn't exist
        let dir_exists = tokio::fs::metadata(dir_path.clone()).await.is_ok();
        assert!(!dir_exists);

        let user = User::new("pub_key".to_string(), "hash".to_string());
        match user_client.put_user(&user).await {
            Ok(result) => {
                // the set user should be a new uuid
                assert!(!result.uuid.is_empty());
                assert_eq!(result.pub_key, user.pub_key);
                assert_eq!(result.hash, user.hash);
                let file_path = format!("{}/user:{}", dir_path.clone(), result.uuid);
                let file_exists = tokio::fs::metadata(file_path).await.is_ok();
                assert!(file_exists);
            },
            Err(_) => assert!(false)
        }

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }

    #[tokio::test]
    async fn test_put_existing_user() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/put_existing_user", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let user_client = UserCLient::new(uri);

        // check that dir_path doesn't exist
        let dir_exists = tokio::fs::metadata(dir_path.clone()).await.is_ok();
        assert!(!dir_exists);

        let user = User::new("pub_key".to_string(), "hash".to_string());
        if let Ok(result) = user_client.clone().put_user(&user).await {
            // the set user should be a new uuid
            assert!(!result.uuid.is_empty());
            assert_eq!(result.pub_key, user.pub_key);
            assert_eq!(result.hash, user.hash);
            let file_path = format!("{}/user:{}", dir_path.clone(), result.uuid);
            let file_exists = tokio::fs::metadata(file_path).await.is_ok();
            assert!(file_exists);

            // put result now
            // it should have a different uuid now and a different file should be created
            if let Ok(new_result) = user_client.clone().put_user(&result).await {
                assert_ne!(new_result.uuid, result.uuid);
                assert_eq!(new_result.pub_key, result.pub_key);
                assert_eq!(new_result.hash, result.hash);
                let file_path = format!("{}/user:{}", dir_path.clone(), new_result.uuid);
                let file_exists = tokio::fs::metadata(file_path).await.is_ok();
                assert!(file_exists);
            } else {
                assert!(false);
            }

        } else {
            assert!(false);
        }

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }

    #[tokio::test]
    async fn test_update_hash_user_doesnt_exist() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/update_hash_user_doesnt_exist", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let user_client = UserCLient::new(uri);

        // check that dir_path doesn't exist
        let dir_exists = tokio::fs::metadata(dir_path.clone()).await.is_ok();
        assert!(!dir_exists);

        let user = User::new("pub_key".to_string(), "hash".to_string());

        let result = user_client.update_hash(&user, "new_hash".to_string()).await.is_err();
        assert!(result);
    }

    #[tokio::test]
    async fn test_update_hash() {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let dir_path = format!("{}/update_hash_user_doesnt_exist", current_directory.display());
        let uri = Uri::builder().path_and_query(dir_path.clone()).build().unwrap();

        let user_client = UserCLient::new(uri);

        // check that dir_path doesn't exist
        let dir_exists = tokio::fs::metadata(dir_path.clone()).await.is_ok();
        assert!(!dir_exists);

        let user = User::new("pub_key".to_string(), "hash".to_string());

        if let Ok(result) = user_client.clone().put_user(&user).await {
            // the set user should be a new uuid
            assert!(!result.uuid.is_empty());
            assert_eq!(result.pub_key, user.pub_key);
            assert_eq!(result.hash, user.hash);
            let file_path = format!("{}/user:{}", dir_path.clone(), result.uuid);
            let file_exists = tokio::fs::metadata(file_path).await.is_ok();
            assert!(file_exists);

            // both uuid and hash should have changed
            match user_client.clone().update_hash(&result, "new_hash".to_string()).await {
                Ok(new_result) => {
                    assert_eq!(new_result.pub_key, result.pub_key);
                    assert_ne!(new_result.uuid, result.uuid);
                    assert_eq!(new_result.hash, "new_hash".to_string());
                    assert_ne!(new_result.hash, result.hash);
                    let file_path = format!("{}/user:{}", dir_path.clone(), new_result.uuid);
                    let file_exists = tokio::fs::metadata(file_path).await.is_ok();
                    assert!(file_exists);
                }
                Err(_) => assert!(false)
            }
        } else {
            assert!(false);
        }

        // clean up
        tokio::fs::remove_dir_all(dir_path.clone()).await.expect("Failed to remove directory");
    }
}