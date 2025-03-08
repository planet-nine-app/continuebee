use std::{str::FromStr, sync::Arc};

use anyhow::anyhow;
use axum::{extract::State, http::StatusCode, Json};
use sessionless::{secp256k1::PublicKey, Sessionless, Signature};

use crate::config::AppState;

use super::{CreateUserRequest, Response};


pub async fn create_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateUserRequest>,
) -> Json<Response> { 
    let message = format!("{}{}{}", body.timestamp, body.pubKey, body.hash);
    let sessionless = Sessionless::new();

    if let Ok(pub_key) = PublicKey::from_str(body.pubKey.as_str()) {
        let sig = match Signature::from_str(body.signature.as_str()) {
            Ok(s) => s,
            Err(_) => {
                return Json(Response::auth_error());
            }
        };

        match sessionless.verify(message, &pub_key, &sig) {
            Ok(()) => {},
            Err(_) => {
                return Json(Response::auth_error());
            }
        };

        // TODO: 
        return Json(Response::success("todo".to_string()))
    } else {
        return Json(Response::auth_error());
    }
}