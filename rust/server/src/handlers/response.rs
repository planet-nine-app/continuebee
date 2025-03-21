use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone, Deserialize)]
pub enum Response {
    User { user_uuid: String },
    Error { code: u16, message: String },
    Success { code: u16 }
}

impl Response {
    pub fn auth_error() -> Self {
        return Response::Error { code: StatusCode::FORBIDDEN.as_u16(), message: "Auth Error".to_string() };
    }

    pub fn user_success(user_uuid: String) -> Self {
        return Response::User { user_uuid: user_uuid }
    }

    pub fn server_error(message: String) -> Self {
        return Response::Error { code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), message: message };
    }

    pub fn not_found() -> Self {
        return Response::Error { code: StatusCode::NOT_FOUND.as_u16(), message: "Not Found".to_string() };
    }

    pub fn not_acceptable() -> Self {
        return Response::Error { code: StatusCode::NOT_ACCEPTABLE.as_u16(), message: "Not Acceptable".to_string() };
    }

    pub fn success(code: u16) -> Self {
        return Response::Success { code: code };
    }
}