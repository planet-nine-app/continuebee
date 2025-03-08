use axum::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum Response {
    User { user_uuid: String },
    Error { code: u16, message: String }
}

impl Response {
    pub fn auth_error() -> Self {
        return Response::Error { code: StatusCode::FORBIDDEN.as_u16(), message: "Auth Error".to_string() };
    }

    pub fn success(user_uuid: String) -> Self {
        return Response::User { user_uuid: user_uuid }
    }
}