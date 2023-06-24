use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde_json::{Value, json};

use sqlx::PgPool;

use crate::errors::CustomError;

use crate::models::message::{CreateMessageSchema, MessageModel };

pub async fn list_messages(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM message ".to_string();

    let task: Vec<MessageModel> = sqlx::query_as::<_, MessageModel>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<MessageModel>, CustomError> {
    let sql = "SELECT * FROM message where id = ($1)".to_string();

    let message: MessageModel = sqlx::query_as::<_, MessageModel>(&sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::MessageNotFound)?;

    Ok(Json(message))
}

pub async fn delete_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "DELETE FROM message WHERE id = ($1)".to_string();

    let _ = sqlx::query(&sql)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::MessageNotFound)?;

    Ok((StatusCode::OK, Json(json!({"message": "Message deleted"}))))
}

pub async fn update_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>,
) -> Result<(StatusCode, Json<CreateMessageSchema>), CustomError> {
    sqlx::query("UPDATE message SET message=$1 WHERE id=$2")
        .bind(&message.message)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::OK, Json(message)))
}

pub async fn create_message(
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>,
) -> Result<(StatusCode, Json<CreateMessageSchema>), CustomError> {
    if message.message.is_empty(){
        return Err(CustomError::BadRequest);
    }

    let sql = "INSERT INTO message (message) values ($1)".to_string();

    sqlx::query(&sql)
        .bind(&message.message)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(message)))
}
