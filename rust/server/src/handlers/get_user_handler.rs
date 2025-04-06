use std::{str::FromStr, sync::Arc};
use axum::{extract::{Path, Query, State}, Json};
use sessionless::{secp256k1::PublicKey, Sessionless, Signature};

use crate::config::AppState;

use super::{QueryParams, Response};



// Returns whether last saved hash matches sent hash
// for a given user_uuid
pub async fn get_user_handler(
    State(data): State<Arc<AppState>>,
    Path(uuid): Path<String>,
    Query(query): Query<QueryParams>,
) -> Json<Response> {

    let user_uuid = uuid.to_string();
    let timestamp  = query.timestamp.to_string();
    let hash = query.hash.to_string();
    let signature = query.signature.to_string();
    let message = format!("{}{}{}", timestamp, user_uuid, hash);

    // get user from user_uuid
     match data.user_client.clone().get_user(&user_uuid).await {
        Some(found_user) => {
            let sessionless = Sessionless::new();

            if let Ok(pub_key) = PublicKey::from_str(found_user.pub_key.as_str()) {
                let sig = match Signature::from_str(signature.as_str()) {
                    Ok(s) => s,
                    Err(_) => {
                        return Json(Response::auth_error());
                    }
                };
                
                // Verify with query params and user's pub_key
                if sessionless.verify(message, &pub_key, &sig).is_err() {
                    return Json(Response::auth_error());
                }
            } else {
                return Json(Response::auth_error());
            }
            
            if found_user.hash == hash {
                return Json(Response::user_success(found_user.uuid));
            } else {
                return Json(Response::not_acceptable());
            }
        },
        None => Json(Response::not_found())
     }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sessionless::Sessionless;

    use crate::{handlers::{QueryParams, Response}, test_common::{cleanup_test_files, setup_test_server, storage_uri, write_user}};


    #[tokio::test]
    async fn test_get_user_handler() {
        let inital_uuid = "1234";
        let initial_hash = "initial_hash";
        let timestamp = Utc::now().timestamp().to_string();
        let get_user_path = format!("/user/{}", inital_uuid);

        let storage_uri = storage_uri("test_get_user_handler");
        let test_server = setup_test_server(storage_uri.clone());

        assert!(test_server.is_running());
        let sessionless = Sessionless::new();
        let pub_key = sessionless.public_key();


        // create directory
        assert!(tokio::fs::create_dir_all(&storage_uri.to_string()).await.is_ok());

        // write user to file
        assert!(write_user(&storage_uri.to_string(), inital_uuid, &pub_key.to_string(), initial_hash).await);

        let message = format!("{}{}{}", timestamp, &inital_uuid, initial_hash);
        let signature = sessionless.sign(message);

        let query_param = QueryParams {
            timestamp: timestamp.to_string(),
            hash: initial_hash.to_string(),
            signature: signature.to_string()
        };

        let response = test_server.get(&get_user_path).add_query_params(&query_param).await;
        assert_eq!(response.status_code(), 200);
        let user_response = response.json::<Response>();
        match user_response {
            Response::User { user_uuid } => {
                assert_eq!(user_uuid, inital_uuid);
            },
            _ => {
                assert!(false);
            }
        }

        // get a user that does not exist
        let get_user_path = format!("/user/{}", "non_existent_user");
        let response = test_server.get(&get_user_path).add_query_params(&query_param).await;
        let user_response = response.json::<Response>();
        match user_response {
            Response::Error { code , message } => {
                assert_eq!(code, 404);
                assert_eq!(message, "Not Found");
            },
            _ => {
                assert!(false);
            }
        }

        // get a user but have the wrong hash
        let get_user_path = format!("/user/{}", inital_uuid);
        let wrong_hash = "wrong_hash";
        let query_param = QueryParams {
            timestamp: timestamp.to_string(),
            hash: wrong_hash.to_string(),
            signature: signature.to_string()
        };
        let response = test_server.get(&get_user_path).add_query_params(&query_param).await;
        let user_response = response.json::<Response>();
        match user_response {
            Response::Error { code , message } => {
                assert_eq!(code, 403);
                assert_eq!(message, "Auth Error");
            },
            _ => {
                assert!(false);
            }
        }
        
        // clean up test files
        cleanup_test_files(&storage_uri.to_string()).await;
    }
}