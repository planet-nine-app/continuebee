use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub pub_key: String,
    pub hash: String,
}

impl User {
    // Create a new user with an empty uuid
    pub fn new(pub_key: String, hash: String) -> Self {
        Self {uuid: "".to_string(), pub_key: pub_key, hash: hash}
    }
}