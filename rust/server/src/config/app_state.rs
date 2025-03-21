
use crate::storage::UserCLient;


#[derive(Debug, Clone)]
pub struct AppState {
    pub user_client: UserCLient,
}