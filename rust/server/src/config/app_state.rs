
use crate::storage::UserCLient;

use super::ServerConfig;

pub struct AppState {
    pub user_client: UserCLient,
    pub env: ServerConfig,
}