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