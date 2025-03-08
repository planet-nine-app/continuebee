use std::str::FromStr;

use axum::http::Uri;
use dotenv::dotenv;


#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub subdomain: String,
    pub port: u16,
    pub storage_uri: Uri,
}

impl ServerConfig {

    pub fn from_env() -> Self {
        dotenv().ok();

        let subdomain = std::env::var("SUBDOMAIN").unwrap_or("localhost".to_string());

        let port = std::env::var("PORT").unwrap_or("3000".to_string());
        let port = port.parse::<u16>().expect("PORT must be a number");

        let storage_uri = std::env::var("STORAGE_URI").expect("STORAGE_URI must be set");
        let storage_uri = Uri::from_str(&storage_uri).expect("STORAGE_URI must be a valid URI");

        ServerConfig {
            subdomain,
            port,
            storage_uri,
        }
    }

    pub fn server_url(self) -> String {
        format!("http://{}:{}", self.subdomain, self.port)
    }
}