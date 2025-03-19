use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub timestamp: String,
    pub hash: String,
    pub signature: String,
}