use std::{str::FromStr, sync::Arc};

use axum::{extract::State, Json};
use sessionless::{secp256k1::PublicKey, Sessionless, Signature};

use crate::{config::AppState, storage::User};

use super::{CreateUserRequest, Response};


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

        let user_to_put = User::new(body.pub_key.clone(), body.hash.clone());
        match data.user_client.clone().put_user(&user_to_put).await {
            Ok(user ) => Json(Response::success(user.uuid)),
            Err(_) => Json(Response::server_error("Failed to put user".to_string()))
        }
    } else {
        return Json(Response::auth_error());
    }
}