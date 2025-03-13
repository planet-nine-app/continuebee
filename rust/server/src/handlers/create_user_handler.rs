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

    use sessionless::PublicKey;
    use std::str::FromStr;

    #[test]
    fn test_pub_key_empty() {
        let pub_key = PublicKey::from_str("");
        println!("{:?}", pub_key.is_ok());
    }
}