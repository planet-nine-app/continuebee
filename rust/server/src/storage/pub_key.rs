use std::collections::HashMap;

use serde::{Serialize, Deserialize};


// Associates a user uuid to a pub_key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PubKeys {
    // {pub_key: user_uuid}
    pub_keys: HashMap<String, String>
}

impl PubKeys {
    
    pub fn default() -> Self {
        Self { pub_keys: HashMap::new() }
    }

    pub fn add_user_uuid(&mut self, user_uuid: &str, pub_key: &str) -> &Self {
        self.pub_keys.insert(pub_key.to_string(), user_uuid.to_string());
        self
    }

    pub fn get_user_uuid(&self, pub_key: &str) -> Option<&String> {
        self.pub_keys.get(pub_key)
    }
}

