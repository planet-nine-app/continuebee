use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CreateUserRequest {
    pub pubKey: String,
    pub hash: String,
    pub timestamp: String,
    pub signature: String,
}