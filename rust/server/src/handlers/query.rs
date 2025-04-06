use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct QueryParams {
    pub timestamp: String,
    pub hash: String,
    pub signature: String,
}