use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sessionless::{Sessionless, Signature};

use crate::config::AppState;

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

    match data.user_client.clone().update_hash(&found_user, body.new_hash).await {
        Ok(new_user) => Json(Response::success(new_user.uuid)),
        Err(_) => Json(Response::server_error("Failed to update hash".to_string()))
    }
}