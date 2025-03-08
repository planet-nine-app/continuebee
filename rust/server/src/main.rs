mod config;
mod storage;
mod handlers;

use std::sync::Arc;
use axum::{routing::{get, post}, Router};

use config::{AppState, ServerConfig};
use handlers::create_user_handler;
use storage::Client;


#[tokio::main]
async fn main() {
    let server_config = ServerConfig::from_env();

    let app = setup_router(&server_config);
    let listener = tokio::net::TcpListener::bind(server_config.server_url()).await.expect("Failed to bind to port");
    axum::serve(listener, app).await.expect("Server failed to start");
}

fn setup_router(server_config: &ServerConfig) -> Router {
    let client = Client::new(server_config.storage_uri.clone());

    let app_state = Arc::new(AppState {
        client: client,
        env: server_config.clone(),
    });

    Router::new()
        .route("/heath_check", get(health_check))
        .route("/user/create", post(create_user_handler))
        .with_state(app_state)
}

async fn health_check() -> String {
    return "Success".to_string();
}