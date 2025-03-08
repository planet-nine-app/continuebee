use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserRequest {
    pub pub_key: String,
    pub hash: String,
    pub timestamp: String,
    pub signature: String,
}