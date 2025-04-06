use std::collections::HashMap;

use serde::{Serialize, Deserialize};


// Associates a user uuid to a pub_key
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PubKeys {
    // {pub_key + hash: user_uuid}
    pub_keys: HashMap<String, String>
}

impl PubKeys {
    
    pub fn default() -> Self {
        Self { pub_keys: HashMap::new() }
    }
    pub fn key(hash: &str, pub_key: &str) -> String {
        format!("{}{}", hash, pub_key)
    }

    pub fn add_user_uuid(&mut self, user_uuid: &str,  key: &str) -> &mut Self {
        self.pub_keys.insert(key.to_string(), user_uuid.to_string());
        self
    }

    pub fn get_user_uuid(&self, key: &str) -> Option<&String> {
        self.pub_keys.get(key)
    }

    pub fn remove_key(&mut self, key: &str) -> Option<String> {
        self.pub_keys.remove(key)
    }

    pub fn num_keys(&self) -> usize {
        self.pub_keys.len()
    }
}

