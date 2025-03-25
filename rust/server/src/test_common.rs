use std::sync::Arc;

use axum::{http::Uri, routing::{delete, get, post, put}, Router};
use axum_test::TestServer;

use crate::{config::AppState, handlers, storage::UserCLient};

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
    let test_user_client = UserCLient::new(storage_uri.clone());

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

pub async fn check_path_exists(path: &str) -> bool {
    tokio::fs::metadata(path).await.is_ok()
}

pub async fn cleanup_test_files(dir: &str) {
    tokio::fs::remove_dir_all(dir).await.expect("Failed to remove test files");
}