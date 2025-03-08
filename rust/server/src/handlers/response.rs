use axum::{http::StatusCode, Json};

#[derive(Debug)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub error: String,
}

impl ErrorResponse {
    pub fn new(status_code: StatusCode, error: String) -> Self {
        ErrorResponse {
            status_code,
            error,
        }
    }
}

impl Into<(StatusCode, Json<serde_json::Value>)> for ErrorResponse {
    fn into(self) -> (StatusCode, Json<serde_json::Value>) {
        let json = serde_json::json!({
            "error": self.error,
        });

        (self.status_code, Json(json))
    }
}