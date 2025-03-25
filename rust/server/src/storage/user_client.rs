use axum::http::Uri;

use super::{Client, PubKeys, StorageClient, User};


static USER_STRING: &str = "user";
static KEYS_STRING: &str = "keys";

#[derive(Debug, Clone)]
pub struct UserCLient {
    pub client: Client
}

impl UserCLient {
    pub fn new(storage_uri: Uri) -> Self {
        Self { client: Client::new(storage_uri) }
    }

    fn user_key(uuid: &str) -> String {
        format!("{}:{}", USER_STRING, uuid)
    }

    pub async fn get_user_uuid(self, key: &str) -> Option<String> {
        match self.get_keys().await {
            Ok(pub_keys) => pub_keys.get_user_uuid(key).cloned(),
            Err(_) => None
        }
    }

    pub async fn get_user(self, uuid: impl AsRef<str>) -> Option<User> {
        match self.client.get(UserCLient::user_key(uuid.as_ref()).as_str()).await {
            Some(value) => {
                match serde_json::from_value(value) {
                    Ok(user) => Some(user),
                    Err(_) => None
                }
            },
            None => None
        }
    }

    // Will put a new user with the given uuid, pub_key, and hash
    // will return the newly put user
    pub async fn put_user(&self, uuid: &str, pub_key: &str, hash: &str) -> anyhow::Result<User> {
        let user = User::new(Some(uuid.to_string()), pub_key.to_string(), hash.to_string());
        if let Ok(value) = serde_json::to_value(user.clone()) {
            match self.client.set(&UserCLient::user_key(&user.uuid).as_str(), value).await {
                Ok(_) => {
                    return Ok(user.clone());
                },
                Err(e) => Err(e.into()),
            }
        } else {
            Err(anyhow::Error::msg("Failed to serialize user"))
        }
    }

    pub async fn delete_user(self, uuid: &str) -> bool {
        self.client.delete(UserCLient::user_key(uuid).as_str()).await
    }

    pub async fn save_pub_keys(&self, keys: PubKeys) -> anyhow::Result<()> {
        if let Ok(value) = serde_json::to_value(keys) {
            self.client.set(KEYS_STRING, value).await?;
            Ok(())
        } else {
            Err(anyhow::Error::msg("Failed to set keys"))
        }
    }

    pub async fn get_keys(&self) -> anyhow::Result<PubKeys> {
        match self.client.get(KEYS_STRING).await {
            Some(value) => {
                match serde_json::from_value(value) {
                    Ok(result) => Ok(result),
                    Err(_) => Ok(PubKeys::default())
                }
            },
            None => Ok(PubKeys::default())
        }
    }

    // will add a new key
    pub async fn update_keys(&self, key: &str, user_uuid: &str) -> anyhow::Result<()> {
        match self.get_keys().await {
            Ok(mut pub_keys) => {
                let pub_keys = pub_keys.add_user_uuid(user_uuid, key);
                self.save_pub_keys(pub_keys.clone()).await
            },
            Err(e) => Err(e)
        }
    }

    pub async fn remove_key(&self, key: &str) -> anyhow::Result<()> {
        match self.get_keys().await {
            Ok(mut pub_keys) => {
                pub_keys.remove_key(key);
                self.save_pub_keys(pub_keys.clone()).await
            },
            Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use sessionless::Sessionless;
    use tokio::io::AsyncWriteExt;
    use crate::test_common::{storage_uri, check_path_exists, cleanup_test_files};

    #[tokio::test]
    async fn test_get_user() {
        let uri = storage_uri("get_user");

        let initial_uuid = "uuid";
        let file_path = format!("{}/user:{}", &uri.to_string(), initial_uuid);
        let user_client = UserCLient::new(uri.clone());

        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        // confirm file doesn't exist before
        assert!(!check_path_exists(&file_path).await);

        let user = User::new(Some(initial_uuid.to_string()), "pub_key".to_string(), "hash".to_string());

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
        cleanup_test_files(&uri.to_string()).await;
    }


    #[tokio::test]
    async fn test_put_user() {
        let uri = storage_uri("put_user");

        let user_client = UserCLient::new(uri.clone());

        // check that dir_path doesn't exist
        check_path_exists(&uri.to_string()).await;

        let pub_key = "pub_key";
        let hash = "hash";
        let uuid = "uuid";
        match user_client.put_user(uuid, pub_key, hash).await {
            Ok(result) => {
                assert_eq!(result.uuid, uuid);
                assert_eq!(result.pub_key.to_string(), pub_key.to_string());
                assert_eq!(result.hash, hash);
                let file_path = format!("{}/user:{}", uri.clone().to_string(), result.uuid);
                assert!(check_path_exists(&file_path).await);
            },
            Err(_) => assert!(false)
        }

        // update the hash of the user
        let new_hash = "new_hash";
        match user_client.put_user(uuid, pub_key, new_hash).await {
            Ok(result) => {
                assert_eq!(result.uuid, uuid);
                assert_eq!(result.pub_key.to_string(), pub_key.to_string());
                assert_eq!(result.hash, new_hash);
                let file_path = format!("{}/user:{}", uri.clone().to_string(), result.uuid);
                assert!(check_path_exists(&file_path).await);
            },
            Err(_) => assert!(false)
        }

        // clean up
        cleanup_test_files(&uri.to_string()).await;
    }

    #[tokio::test]
    async fn test_delete_user() {
        let uri = storage_uri("delete_user");

        let initial_uuid = "uuid";
        let file_path = format!("{}/user:{}", &uri.to_string(), initial_uuid);
        let user_client = UserCLient::new(uri.clone());

        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        // confirm the file doesn't exist before
        assert!(!check_path_exists(&file_path).await);

        let user = User::new(Some(initial_uuid.to_string()), "pub_key".to_string(), "hash".to_string());
        let data = serde_json::to_value(user.clone()).expect("Failed to serialize");

        // write user to file with fs::write
        let mut file = match tokio::fs::File::create_new(file_path.clone()).await {
            Ok(file) => file,
            Err(e) => panic!("Failed to write to file: {}", e),
        };

        assert!(file.write_all(serde_json::to_string(&data).expect("Failed to serialize to string").as_bytes()).await.is_ok());

        // confirm the file exists
        assert!(check_path_exists(&file_path).await);

        // delete the user: should be true as the file should be deleted
        assert!(user_client.clone().delete_user(initial_uuid).await);

        // confirm the file doesn't exist after
        assert!(!check_path_exists(&file_path).await);

        // try to delete the user again: should be false as the file doesn't exist
        assert!(!user_client.clone().delete_user(initial_uuid).await);

        // clean up
        cleanup_test_files(&uri.to_string()).await;
    }

    #[tokio::test]
    async fn test_get_keys() {
        let uri = storage_uri("get_keys");

        let file_path = format!("{}/{}", &uri.to_string(), KEYS_STRING);
        let user_client = UserCLient::new(uri.clone());

        // confirm file doesn't exist before
        assert!(!check_path_exists(&file_path).await);

        // Keys are default when the file doesn't exist
        match user_client.get_keys().await {
            Ok(result) => {
                assert_eq!(result, PubKeys::default());
            },
            Err(_) => assert!(false)
        }

        // create directory
        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        let user_uuid = "test_user_uuid";
        let pub_key = "test_pub_key";

        let mut pub_keys = PubKeys::default();
        let pub_keys = pub_keys.add_user_uuid(user_uuid, pub_key);
        let data = serde_json::to_value(pub_keys.clone()).expect("Failed to serialize");

        // write pub_keys to file with fs::write
        let mut file = match tokio::fs::File::create_new(file_path).await {
            Ok(file) => file,
            Err(e) => panic!("Failed to write to file: {}", e),
        };

        assert!(file.write_all(serde_json::to_string(&data).expect("Failed to serialize to string").as_bytes()).await.is_ok());

        match user_client.clone().get_keys().await {
            Ok(result) => {
                let result_user_uuid = result.get_user_uuid(pub_key);
                assert!(result_user_uuid.is_some());
                assert_eq!(user_uuid, result_user_uuid.unwrap().as_str());
            },
            Err(_) => assert!(false)
        };

        // clean up
        cleanup_test_files(&uri.to_string()).await;
    }

    #[tokio::test]
    async fn test_save_pub_keys() {
        let uri = storage_uri("save_pub_keys");

        let file_path = format!("{}/{}", &uri.to_string(), KEYS_STRING);
        let user_client = UserCLient::new(uri.clone());

        // confirm file doesn't exist before
        assert!(!check_path_exists(&file_path).await);

        // create directory
        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        let mut pub_keys = PubKeys::default();
        let pub_keys = pub_keys.add_user_uuid("test_user_uuid", "test_pub_key");

        match user_client.clone().save_pub_keys(pub_keys.clone()).await {
            Ok(_) => {
                assert!(check_path_exists(&file_path).await);
                // read the file and check the contents
                match tokio::fs::read(file_path.clone()).await {
                    Ok(data) => {
                        let result: PubKeys = serde_json::from_slice(data.as_slice()).expect("Failed to deserialize");
                        assert_eq!(result, *pub_keys);
                    },
                    Err(e) => panic!("Failed to read file: {}", e)
                }
            },
            Err(_) => assert!(false)
        }

        // clean up
        cleanup_test_files(&uri.to_string()).await;
    }

    #[tokio::test]
    async fn test_update_keys() {
        let uri = storage_uri("update_keys");

        let file_path = format!("{}/{}", &uri.to_string(), KEYS_STRING);
        let user_client = UserCLient::new(uri.clone());

        // confirm file doesn't exist before
        assert!(!check_path_exists(&file_path).await);

        // create directory
        match user_client.clone().client {
            Client::FileStorageClient { storage_client } => {
                storage_client.create_storage_dir().await.expect("Failed to create storage directory");
            },
            _ => assert!(false)
        }

        let sessionless = Sessionless::new();
        let pub_key = sessionless.public_key();
        let hash = "hash";
        let user_uuid = "test_user_uuid";

        let key = PubKeys::key(hash, &pub_key.to_string());
        match user_client.clone().update_keys(&key, user_uuid).await {
            Ok(_) => {
                assert!(check_path_exists(&file_path).await);
                // read the file and check the contents
                match tokio::fs::read(file_path.clone()).await {
                    Ok(data) => {
                        let result: PubKeys = serde_json::from_slice(data.as_slice()).expect("Failed to deserialize");
                        let result_user_uuid = result.get_user_uuid(&key.to_string());
                        assert!(result_user_uuid.is_some());
                        assert_eq!(user_uuid, result_user_uuid.unwrap().as_str());
                    },
                    Err(e) => panic!("Failed to read file: {}", e)
                }
            },
            Err(_) => assert!(false)
        }

        // put another user with same pub_key but different hash
        let diff_hash = "diff_hash";
        let diff_key = PubKeys::key(diff_hash, &pub_key.to_string());
        let diff_uuid = "diff_user_uuid";
        match user_client.clone().update_keys(&diff_key, diff_uuid).await {
            Ok(_) => {
                assert!(check_path_exists(&file_path).await);
                // read the file and check the contents
                match tokio::fs::read(file_path.clone()).await {
                    Ok(data) => {
                        let result: PubKeys = serde_json::from_slice(data.as_slice()).expect("Failed to deserialize");
                        let result_user_uuid = result.get_user_uuid(&diff_key.to_string());
                        assert!(result_user_uuid.is_some());
                        assert_eq!(diff_uuid, result_user_uuid.unwrap().as_str());
                    },
                    Err(e) => panic!("Failed to read file: {}", e)
                }
            },
            Err(_) => assert!(false)
        }

        // TODO add other test cases

        // clean up
        cleanup_test_files(&uri.to_string()).await;
    }
}