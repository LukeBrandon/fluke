use std::fmt;

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub enum CustomError {
    BadRequest,
    MessageNotFound,
    ChannelNotFound,
    UserNotFound(String),
    InternalServerError,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Self::BadRequest => (StatusCode::BAD_REQUEST, "Bad Request".to_string()),
            Self::MessageNotFound => (StatusCode::NOT_FOUND, "Message Not Found".to_string()),
            Self::ChannelNotFound => (StatusCode::NOT_FOUND, "Channel Not Found".to_string()),
            Self::UserNotFound(user_param) => {
                let msg = format!("User Not Found: {}", user_param);
                (StatusCode::NOT_FOUND, msg)
            }
        };
        (status, Json(json!({ "error": error_message }))).into_response()
    }
}

#[derive(Debug)]
pub enum SignupError {
    NonUniqueIdError,
    UnknownQueryError,
    UnknownDatabaseError,
}

impl From<sqlx::Error> for SignupError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db_error) => {
                let pg_error = db_error.downcast::<sqlx::postgres::PgDatabaseError>();
                match pg_error.code() {
                    "23505" => {
                        println!("Duplicate user ID.");
                        SignupError::NonUniqueIdError
                    }
                    _ => {
                        println!("-- An error the server didn't account for --");
                        println!("{:?}", pg_error);
                        SignupError::UnknownQueryError
                    }
                }
            }
            _ => {
                println!("Something else happened");
                SignupError::UnknownDatabaseError
            }
        }
    }
}

impl std::fmt::Display for SignupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignupError::NonUniqueIdError => {
                write!(f, "Duplicate email contained a duplicate key.")
            }
            SignupError::UnknownQueryError => {
                write!(f, "Database query contained an unspecified error.")
            }
            SignupError::UnknownDatabaseError => write!(f, "Database error, not query related."),
        }
    }
}
