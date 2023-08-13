use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use sqlx::{error::DatabaseError, Error as SqlxError};

pub enum CustomError {
    BadRequest(String),
    InternalServerError,
}

/// Converts `sqlx::Error` into `CustomError`.
///
/// Enables using the `?` operator in functions returning `Result<T, CustomError>`
/// when interacting with functions that return `Result<T, sqlx::Error>`.
///
/// # Example
///
/// ```ignore
/// fn my_function() -> Result<(), CustomError> {
///     another_function()?;  // This function returns `Result<(), sqlx::Error>`
///     Ok(())
/// }
/// ```
impl From<SqlxError> for CustomError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::Database(db_error) => {
                let pg_error = db_error.downcast::<sqlx::postgres::PgDatabaseError>();
                if pg_error.is_unique_violation() {
                    return CustomError::BadRequest(format!(
                        "Unique violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_check_violation() {
                    return CustomError::BadRequest(format!(
                        "Check violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_foreign_key_violation() {
                    return CustomError::BadRequest(format!(
                        "Foreign violation: {}",
                        pg_error.message()
                    ));
                } else if pg_error.is_unique_violation() {
                    return CustomError::BadRequest(format!(
                        "Duplicate key: {}",
                        pg_error.message()
                    ));
                }
                CustomError::BadRequest("A database error occurred".to_string())
            }
            _ => CustomError::InternalServerError,
        }
    }
}

/// Converts `CustomError` into an HTTP `Response`.
///
/// Each `CustomError` variant is mapped to an appropriate HTTP status code and
/// error message, ensuring consistent and meaningful error responses to clients.
///
/// # Examples
///
/// ```ignore
/// // Given an instance of `CustomError::InternalServerError`:
/// let error = CustomError::InternalServerError;
/// let response = error.into_response();
/// assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
/// ```
///
/// ```ignore
/// // For a database unique violation:
/// let pg_error = sqlx::Error::Database(/*... simulated unique violation error ...*/);
/// let custom_error = CustomError::from(pg_error);
/// let response = custom_error.into_response();
/// assert_eq!(response.status(), StatusCode::BAD_REQUEST);
/// ```
impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            CustomError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
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
