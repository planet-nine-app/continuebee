
use crate::storage::UserClient;


#[derive(Debug, Clone)]
pub struct AppState {
    pub user_client: UserClient,
}