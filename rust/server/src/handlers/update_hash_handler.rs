use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sessionless::{Sessionless, Signature};

use crate::{config::AppState, storage::PubKeys};

use super::{Response, UpdateHashRequest};


pub async fn update_hash_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateHashRequest>,
) -> Json<Response> {
    let message = format!("{}{}{}{}", body.timestamp, body.user_uuid, body.hash, body.new_hash);
    let sessionless = Sessionless::new();

    let sig = match Signature::from_str(body.signature.as_str()) {
        Ok(s) => s,
        Err(_) => {
            return Json(Response::auth_error());
        }
    };

    let found_user = match data.user_client.clone().get_user(&body.user_uuid).await {
        Some(user) => user,
        None => {
            return Json(Response::not_found());
        }
    };

    let pub_key = match found_user.pub_key() {
        Ok(key) => key,
        Err(_) => {
            return Json(Response::auth_error());
        }
    };

    if sessionless.verify(message, &pub_key, &sig).is_err() {
        return Json(Response::auth_error());
    }

    match data.user_client.put_user(&found_user.uuid, &pub_key.to_string(), &body.new_hash).await {
        Ok(new_user) => {
            // Need to update the pub keys map with the new hash
            let old_key = PubKeys::key(&body.hash, &pub_key.to_string());
            if let Ok(_) = data.user_client.remove_key(&old_key).await {
                let new_key = PubKeys::key(&body.new_hash, &pub_key.to_string());
                if data.user_client.update_keys(&new_key, &new_user.uuid).await.is_err() {
                    return Json(Response::server_error("Failed to update keys".to_string()));
                }             
            } else {
                return Json(Response::server_error("Failed to delete old key".to_string()));
            }
            Json(Response::user_success(new_user.uuid))
        },
        Err(_) => Json(Response::server_error("Failed to update hash".to_string()))
    }

}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use sessionless::Sessionless;

    use crate::handlers::{UpdateHashRequest, Response};
    use crate::storage::PubKeys;
    use crate::test_common::{check_path_exists, cleanup_test_files, read_keys, read_user, setup_test_server, storage_uri, write_keys, write_user, USER_UPDATE_HASH_PATH};


    #[tokio::test]
    async fn test_update_hash() {
        let inital_uuid_1 = "1234";
        let initial_hash_1 = "initial_hash_1";
        let new_hash_1 = "new_hash_1";

        let timestamp = Utc::now().timestamp().to_string();

        let storage_uri = storage_uri("test_update_hash");
        let test_server = setup_test_server(storage_uri.clone());
        let user_file_path_1 = format!("{}/user:{}", &storage_uri.to_string(), inital_uuid_1);
        let key_file_path = format!("{}/keys", &storage_uri.to_string());

        assert!(test_server.is_running());
        let sessionless = Sessionless::new();
        let pub_key = sessionless.public_key();

        // create directory
        assert!(tokio::fs::create_dir_all(&storage_uri.to_string()).await.is_ok());

        // write user to file
        assert!(write_user(&storage_uri.to_string(), inital_uuid_1, &pub_key.to_string(), initial_hash_1).await);
        // and write keys
        let mut pub_keys = PubKeys::default();
        pub_keys
            .add_user_uuid(inital_uuid_1, &PubKeys::key(initial_hash_1, &pub_key.to_string()));
        assert!(write_keys(&storage_uri.to_string(), &pub_keys).await);

        // check if the files exist
        assert!(check_path_exists(&user_file_path_1).await);
        // check if keys exists
        assert!(check_path_exists(&key_file_path).await);

        let message = format!("{}{}{}{}", timestamp, inital_uuid_1, initial_hash_1, new_hash_1);
        let signature = sessionless.sign(message);

        let payload = UpdateHashRequest {
            user_uuid: inital_uuid_1.to_string(),
            timestamp: timestamp,
            hash: initial_hash_1.to_string(),
            new_hash: new_hash_1.to_string(),
            signature: signature.to_string(),
        };

        let response = test_server.put(USER_UPDATE_HASH_PATH).json(&payload).await;
        let update_response = response.json::<Response>();
        match update_response.clone() {
            Response::User { user_uuid } => {
                assert_eq!(user_uuid, inital_uuid_1);
                // verify the user file was updated
                let user = read_user(&storage_uri.to_string(), inital_uuid_1).await.expect("Failed to read user");
                assert!(user.uuid == inital_uuid_1);
                assert_eq!(user.hash, new_hash_1);

                // verify the keys file was updated
                let pub_keys = read_keys(&storage_uri.to_string()).await.expect("Failed to read keys");
                assert!(pub_keys.num_keys() == 1);

                let old_key = PubKeys::key(&initial_hash_1, &pub_key.to_string());
                let expected_key = PubKeys::key(&new_hash_1, &pub_key.to_string());
                let expected_result = pub_keys.get_user_uuid(&expected_key);
                assert!(expected_result.is_some());
                assert_eq!(expected_result.unwrap(), &inital_uuid_1);

                // verify the old key was removed
                let old_key_result = pub_keys.get_user_uuid(&old_key);
                assert!(old_key_result.is_none());
            },
            _ => {
                panic!("Failed to update hash");
            }
        }

        // cleanup
        cleanup_test_files(&storage_uri.to_string()).await;
    }
}