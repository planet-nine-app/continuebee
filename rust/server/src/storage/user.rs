use serde::{Serialize, Deserialize};


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
}