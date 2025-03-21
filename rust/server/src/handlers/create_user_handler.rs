use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sessionless::{secp256k1::PublicKey, Sessionless, Signature};

use crate::config::AppState;

use super::{CreateUserRequest, Response};


// Creates a new user if pubKey does not exist, and returns existing uuid if it does.
// signature message is: timestamp + pubKey + hash
pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> Json<Response> { 
    let message = format!("{}{}{}", body.timestamp, body.pub_key, body.hash);
    let sessionless = Sessionless::new();

    if let Ok(pub_key) = PublicKey::from_str(body.pub_key.as_str()) {
        let sig = match Signature::from_str(body.signature.as_str()) {
            Ok(s) => s,
            Err(_) => {
                return Json(Response::auth_error());
            }
        };

        if sessionless.verify(message, &pub_key, &sig).is_err() {
            return Json(Response::auth_error());
        }

        match data.user_client.clone().get_user_uuid(&pub_key).await {
            // If user exists with given pub_key, return back the user_uuid
            Some(user_uuid) => Json(Response::success(user_uuid)),
            None => {
                // otherwise, put a new user
                match data.user_client.clone().put_user(&body.pub_key, &body.hash).await {
                    Ok(user) => {
                        // add pub key with user uuid
                        match data.user_client.clone().update_keys(&pub_key, &user.uuid).await {
                            Ok(_) => Json(Response::success(user.uuid)),
                            Err(_) => Json(Response::server_error("Failed to update keys".to_string()))
                        }
                    },
                    Err(_) => Json(Response::server_error("Failed to put user".to_string()))
                }
            }
        }
    } else {
        return Json(Response::auth_error());
    }
}


#[cfg(test)]
mod tests {

    use axum::{http::Uri, routing::post, Router};
    use axum_test::TestServer;
    use chrono::Utc;
    use sessionless::Sessionless;
    use std::sync::Arc;

    use crate::{config::AppState, handlers::{create_user_handler, CreateUserRequest, Response}, storage::UserCLient};

    fn storage_uri(test_name: &str) -> Uri {
        let current_directory = std::env::current_dir().expect("Failed to get current directory"); 
        let storage_uri = format!("{}/{}", current_directory.display(), test_name);
        Uri::builder().path_and_query(storage_uri.clone()).build().unwrap()
    }

    fn setup_test_server(storage_uri: Uri) -> TestServer {
        let test_user_client = UserCLient::new(storage_uri.clone());

        let test_app_state = Arc::new( AppState {
            user_client: test_user_client,
        });

        let app = Router::new()
            .route("/user/create", post(create_user_handler))
            .with_state(test_app_state);

        TestServer::new(app).unwrap()
    }

    async fn check_path_exists(path: &str) -> bool {
        tokio::fs::metadata(path).await.is_ok()
    }

    async fn cleanup_test_files(dir: &str) {
        tokio::fs::remove_dir_all(dir).await.expect("Failed to remove test files");
    }

    #[tokio::test]
    async fn test_create_user_handler() {
        let stroage_uri = storage_uri("test_create_user_handler");
        let test_server = setup_test_server(stroage_uri.clone());

        assert!(test_server.is_running());
        let sessionless = Sessionless::new();

        let pub_key = sessionless.public_key();
        let timestamp = Utc::now().timestamp().to_string();
        let hash = "random_hash".to_string();

        let message = format!("{}{}{}", timestamp, pub_key, hash);
        let signature = sessionless.sign(message);

        let payload = CreateUserRequest {
            pub_key: pub_key.to_string(),
            timestamp: timestamp,
            hash: hash,
            signature: signature.to_string(),
        };

        let post_path = "/user/create";

        let response = test_server.post(post_path).json(&payload).await;

        assert_eq!(response.clone().status_code(), 200);
        // get the user_uuid from the response
        // parse as Response
        let user_resposne = response.json::<Response>();

        match user_resposne.clone() {
            Response::User { user_uuid } => {
                assert_eq!(user_uuid.is_empty(), false);
                // check that the user file created exists
                let file_path = format!("{}/user:{}", stroage_uri.to_string(), user_uuid);
                assert!(check_path_exists(file_path.as_str()).await);

                // check the keys file also exists
                let keys_file_path = format!("{}/keys", stroage_uri.to_string());
                assert!(check_path_exists(keys_file_path.as_str()).await);

                // TODO check the keys file has the correct pub_key and user_uuid
            },
            _ => {
                assert!(false);
            }
        }
        cleanup_test_files(&stroage_uri.to_string()).await;
    }
}