use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use serde_json::json;

pub enum CustomError {
    BadRequest,
    MessageNotFound,
    UserNotFound,
    InternalServerError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
            }
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request"),
            Self::MessageNotFound => (StatusCode::NOT_FOUND, "Message Not Found"),
            Self::UserNotFound => (StatusCode::NOT_FOUND, "User Not Found"),
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}
