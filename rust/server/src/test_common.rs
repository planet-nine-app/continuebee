use std::sync::Arc;

use axum::{http::Uri, routing::{delete, get, post, put}, Router};
use axum_test::TestServer;
use tokio::io::AsyncWriteExt;

use crate::{config::AppState, handlers, storage::{PubKeys, User, UserClient}};

pub static USER_CREATE_PATH: &str = "/user/create";
pub static USER_UPDATE_HASH_PATH: &str = "/user/update-hash";
pub static USER_DELETE_PATH: &str = "/user/delete";
pub static USER_GET_PATH: &str = "/user/{uuid}";

pub fn storage_uri(test_name: &str) -> Uri {
    let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
    let storage_uri = format!("{}/{}", current_directory.display(), test_name);
    Uri::builder().path_and_query(storage_uri.clone()).build().unwrap()
}

fn test_router(storage_uri: Uri) -> Router {
    let test_user_client = UserClient::new(storage_uri.clone());

    let test_app_state = Arc::new(AppState {
        user_client: test_user_client,
    });

    Router::new()
        .route(&USER_CREATE_PATH, post(handlers::create_user_handler))
        .route(&USER_GET_PATH, get(handlers::get_user_handler))
        .route(&USER_UPDATE_HASH_PATH, put(handlers::update_hash_handler))
        .route(&USER_DELETE_PATH, delete(handlers::delete_user_handler))
        .with_state(test_app_state)
}

pub fn setup_test_server(storage_uri: Uri) -> TestServer {
    let router = test_router(storage_uri);

    TestServer::new(router).unwrap()
}

pub async fn write_user(dir_path: &str, uuid: &str, pub_key: &str, hash: &str) -> bool {
    let user = User::new(Some(uuid.to_string()), pub_key.to_string(), hash.to_string());
    let data = serde_json::to_value(&user).unwrap();

    let file_path = format!("{}/user:{}", &dir_path, uuid);
    let mut file = match tokio::fs::File::create_new(file_path).await {
        Ok(f) => f,
        Err(e) => {

            panic!("Failed to create file {}", e);
        }
    };

    file.write_all(data.to_string().as_bytes()).await.is_ok()
}

pub async fn write_keys(dir_path: &str, pub_keys: &PubKeys) -> bool {
    let data = serde_json::to_value(pub_keys).unwrap();

    let file_path = format!("{}/keys", &dir_path);
    let mut file = match tokio::fs::File::create_new(file_path).await {
        Ok(f) => f,
        Err(e) => {
            panic!("Failed to create file {}", e);
        }
    };

    file.write_all(data.to_string().as_bytes()).await.is_ok()
}

pub async fn read_keys(dir_path: &str) -> anyhow::Result<PubKeys> {

    let file_path = format!("{}/keys", &dir_path);
    let data = tokio::fs::read_to_string(file_path).await?;

    let pub_keys: PubKeys = serde_json::from_str(&data)?;
    Ok(pub_keys)
}

pub async fn check_path_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}

pub async fn cleanup_test_files(dir: &str) {
    tokio::fs::remove_dir_all(dir).await.expect("Failed to remove test files");
}