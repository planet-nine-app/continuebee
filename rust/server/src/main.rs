mod config;
mod storage;
mod handlers;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};

use config::{AppState, ServerConfig};
use storage::UserCLient;


#[tokio::main]
async fn main() {
    let server_config = ServerConfig::from_env();

    let app = setup_router(&server_config);
    let listener = tokio::net::TcpListener::bind(server_config.server_url()).await.expect("Failed to bind to port");
    axum::serve(listener, app).await.expect("Server failed to start");
}

fn setup_router(server_config: &ServerConfig) -> Router {
    let user_client = UserCLient::new(server_config.storage_uri.clone());

    let app_state = Arc::new(AppState {
        user_client: user_client,
    });

    Router::new()
        .route("/heath_check", get(health_check))
        .route("/user/create", post(handlers::create_user_handler))
        .route("/user/{uuid}", get(handlers::get_user_handler))
        .with_state(app_state)
}

async fn health_check() -> String {
    return "Success".to_string();
}