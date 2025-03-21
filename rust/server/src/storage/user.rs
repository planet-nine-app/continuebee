use std::str::FromStr;

use serde::{Serialize, Deserialize};
use sessionless::secp256k1::PublicKey;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub uuid: String,
    pub pub_key: String,
    pub hash: String,
}

impl User {
    // Create a new user with an empty uuid
    pub fn new(uuid: Option<String>, pub_key: String, hash: String) -> Self {
        match uuid {
            Some(uuid) => Self {uuid: uuid, pub_key: pub_key, hash: hash},
            None => Self {uuid: "".to_string(), pub_key: pub_key, hash: hash}
        }
    }

    pub fn pub_key(&self) -> anyhow::Result<PublicKey> {
        PublicKey::from_str(self.pub_key.as_str()).map_err(|_| anyhow::anyhow!("Failed to parse public key"))
    }
}