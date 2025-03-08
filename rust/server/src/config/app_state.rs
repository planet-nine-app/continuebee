
use crate::storage::UserCLient;

use super::ServerConfig;

#[derive(Debug, Clone)]
pub struct AppState {
    pub user_client: UserCLient,
    pub env: ServerConfig,
}