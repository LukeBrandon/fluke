use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde_json::{json, Value};

use sqlx::PgPool;

use crate::errors::CustomError;

use crate::models::message::{CreateMessageSchema, MessageModel};

pub async fn list_messages(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM message";

    let task: Vec<MessageModel> = sqlx::query_as::<_, MessageModel>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<MessageModel>, CustomError> {
    let sql = "SELECT * FROM message where id = ($1)";

    let message: MessageModel = sqlx::query_as::<_, MessageModel>(sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::MessageNotFound
        })?;

    Ok(Json(message))
}

pub async fn delete_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "DELETE FROM message WHERE id = ($1)";

    let _ = sqlx::query(sql)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::MessageNotFound
        })?;

    Ok((StatusCode::OK, Json(json!({"message": "Message deleted"}))))
}

pub async fn update_message(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>,
) -> Result<(StatusCode, Json<MessageModel>), CustomError> {
    let updated = sqlx::query_as!(
        MessageModel,
        "UPDATE message SET message=$1 WHERE id=$2 RETURNING *",
        &message.message,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::OK, Json(updated)))
}

pub async fn create_message(
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>,
) -> Result<(StatusCode, Json<MessageModel>), CustomError> {
    if message.message.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let created = sqlx::query_as!(
        MessageModel,
        "INSERT INTO message (message, user_id, channel_id)  VALUES ($1. $2, $3) RETURNING *",
        &message.message, &message.user_id, &message.channel_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::CREATED, Json(created)))
}
