use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde_json::{json, Value};

use sqlx::PgPool;

use crate::errors::CustomError;

use crate::models::message::{CreateMessageSchema, UpdateMessageSchema, MessageModel};

pub async fn list_messages(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>
    ) -> impl IntoResponse {
    let sql = "SELECT * FROM message where channel_id = ($1)";

    let task: Vec<MessageModel> = sqlx::query_as::<_, MessageModel>(sql)
        .bind(channel_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_message(
    Path((channel_id , message_id)): Path<(i64, i64)>,
    Extension(pool): Extension<PgPool>,
    ) -> Result<Json<MessageModel>, CustomError> {
    let sql = "SELECT * FROM message where channel_id = ($1) and id = ($2)";

    let message: MessageModel = sqlx::query_as::<_, MessageModel>(sql)
        .bind(channel_id)
        .bind(message_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::MessageNotFound)?;

    Ok(Json(message))
}

pub async fn delete_message(
    Path((channel_id , message_id)): Path<(i64, i64)>,
    Extension(pool): Extension<PgPool>,
    ) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "DELETE FROM message WHERE channel_id = ($1) and  id = ($2)";

    let _ = sqlx::query(sql)
        .bind(channel_id)
        .bind(message_id)
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::OK, Json(json!({"message": "Message deleted"}))))
}

pub async fn update_message(
    Path((channel_id , message_id)): Path<(i64, i64)>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<UpdateMessageSchema>,
    ) -> Result<(StatusCode, Json<MessageModel>), CustomError> {
    let updated = sqlx::query_as!(
        MessageModel,
        "UPDATE message SET message=$1 WHERE channel_id=$2 and id=$3 RETURNING *",
        &message.message,
        channel_id,
        message_id
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
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>,
    ) -> Result<(StatusCode, Json<MessageModel>), CustomError> {
    if message.message.is_empty() {
        return Err(CustomError::InternalServerError);
    }

    let created = sqlx::query_as!(
        MessageModel,
        "INSERT INTO message (channel_id, message, user_id) VALUES ($1, $2, $3) RETURNING *",
        channel_id,
        &message.message,
        &message.user_id
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::InternalServerError
        })?;

    Ok((StatusCode::CREATED, Json(created)))
}
