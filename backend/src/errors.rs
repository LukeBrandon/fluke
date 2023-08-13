use std::fmt;
use axum::{http::StatusCode,
response::{IntoResponse, Response, Json}
};
use serde_json::json;
use sqlx::Error as SqlxError;

pub enum CustomError {
    NotFound(String),
    DatabaseError(SqlxError),
    InternalServerError,
}

impl From<SqlxError> for CustomError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::Database(e) => {
                CustomError::DatabaseError(sqlx::Error::Database(e))
            },
            _ => CustomError::InternalServerError,
        }
    }
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Self::NotFound(detail) => (StatusCode::NOT_FOUND, detail),
            Self::DatabaseError(detail) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database Error: {}", detail),
            ),
        };
         let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()

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
