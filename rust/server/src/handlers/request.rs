use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub pub_key: String,
    pub hash: String,
    pub timestamp: String,
    pub signature: String,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHashRequest {
    pub user_uuid: String,
    pub timestamp: String,
    pub hash: String,
    pub new_hash: String,
    pub signature: String,
}