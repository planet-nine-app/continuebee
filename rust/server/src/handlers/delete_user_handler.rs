


use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sessionless::{Sessionless, Signature};

use crate::{config::AppState, storage::PubKeys};

use super::{DeleteUserRequest, Response};

// Deletes the user from storage and the public key + hash
pub async fn delete_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<DeleteUserRequest>,
) -> Json<Response> {

    let message = format!("{}{}{}", body.timestamp, body.user_uuid, body.hash);
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

    let key = PubKeys::key(&body.hash, &pub_key.to_string());
    if data.user_client.clone().delete_user(&found_user.uuid).await {
        if let Err(_) = data.user_client.clone().remove_key(&key).await {
            return Json(Response::server_error("Failed to delete key".to_string()))
        }

        Json(Response::success(202))
    } else {
        Json(Response::server_error("Failed to delete user".to_string()))
    }

}