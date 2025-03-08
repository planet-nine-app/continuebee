
use crate::storage::Client;

use super::ServerConfig;

pub struct AppState {
    pub client: Client,
    pub env: ServerConfig,
}