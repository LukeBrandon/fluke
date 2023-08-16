use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};

use serde_json::json;
use sqlx::{error::DatabaseError, Error as SqlxError};

pub enum FlukeApiError {
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    InternalServerError,
}

/// Converts `sqlx::Error` into `FlukeApiError`.
///
/// Enables using the `?` operator in functions returning `Result<T, FlukeApiError>`
/// when interacting with functions that return `Result<T, sqlx::Error>`.
///
/// # Example
///
/// ```ignore
/// fn my_function() -> Result<(), FlukeApiError> {
///     another_function()?;  // This function returns `Result<(), sqlx::Error>`
///     Ok(())
/// }
/// ```
impl From<SqlxError> for FlukeApiError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => FlukeApiError::NotFound(err.to_string()),
            SqlxError::Database(db_error) => {
                let pg_error = db_error.downcast::<sqlx::postgres::PgDatabaseError>();
                if pg_error.is_unique_violation() {
                    return FlukeApiError::BadRequest(format!(
                        "Unique violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_check_violation() {
                    return FlukeApiError::BadRequest(format!(
                        "Check violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_foreign_key_violation() {
                    return FlukeApiError::BadRequest(format!(
                        "Foreign violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_unique_violation() {
                    return FlukeApiError::BadRequest(format!(
                        "Duplicate key: {}",
                        pg_error.message()
                    ));
                }
                FlukeApiError::InternalServerError // catch-all for database errors
            }
            _ => FlukeApiError::InternalServerError, // catch-all for sqlx - non-db errors
                                                     // https://docs.rs/sqlx/latest/sqlx/enum.Error.html
        }
    }
}

/// Converts `FlukeApiError` into an HTTP `Response`.
///
/// Each `FlukeApiError` variant is mapped to an appropriate HTTP status code and
/// error message, ensuring consistent and meaningful error responses to clients.
///
/// # Examples
///
/// ```ignore
/// // Given an instance of `FlukeApiError::InternalServerError`:
/// let error = FlukeApiError::InternalServerError;
/// let response = error.into_response();
/// assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
/// ```
///
/// ```ignore
/// // For a database unique violation:
/// let pg_error = sqlx::Error::Database(/*... simulated unique violation error ...*/);
/// let custom_error = FlukeApiError::from(pg_error);
/// let response = custom_error.into_response();
/// assert_eq!(response.status(), StatusCode::BAD_REQUEST);
/// ```
impl IntoResponse for FlukeApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            FlukeApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            FlukeApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            FlukeApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
