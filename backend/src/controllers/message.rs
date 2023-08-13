use axum::extract::{Path, Extension, Json};
use axum::http::StatusCode;
use sqlx::PgPool;
use serde_json::{json, Value};
use crate::errors::CustomError;
use crate::db::Db;
use crate::models::message::MessageModel;
use crate:: models::message::{CreateMessageSchema, UpdateMessageSchema};

pub async fn list_messages(Path(channel_id): Path<i64>, Extension(pool): Extension<PgPool>) -> Result<Json<Vec<MessageModel>>, CustomError> {
    let messages = Db::list_messages(channel_id, &pool).await.map_err(CustomError::from)?;
    Ok(Json(messages))
}

pub async fn get_message(Path((channel_id, message_id)): Path<(i64, i64)>, Extension(pool): Extension<PgPool>) -> Result<Json<MessageModel>, CustomError> {
    let message = Db::get_message(channel_id, message_id, &pool).await.map_err(CustomError::from)?;
    Ok(Json(message))
}

pub async fn delete_message(Path((channel_id , message_id)): Path<(i64, i64)>, Extension(pool): Extension<PgPool>)-> Result<(StatusCode, Json<Value>), CustomError> {
    let _ = Db::delete_message(channel_id, message_id, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(json!({"message": "Message deleted"}))))
}

pub async fn update_message(Path((channel_id, message_id)): Path<(i64, i64)>, Extension(pool): Extension<PgPool>, Json(message): Json<UpdateMessageSchema>)-> Result<(StatusCode, Json<MessageModel>), CustomError> {
    let updated_message = Db::update_message(channel_id, message_id, &message.message, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(updated_message)))
}

pub async fn create_message(Path(channel_id): Path<i64>, Extension(pool): Extension<PgPool>, Json(message): Json<CreateMessageSchema>)-> Result<(StatusCode, Json<MessageModel>), CustomError> {
    let created_message = Db::create_message(channel_id, &message.message, &message.user_id, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(created_message)))
}
