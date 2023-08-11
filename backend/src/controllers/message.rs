// controllers/message.rs
use axum::extract::{Path, Extension, Json};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use sqlx::PgPool;

use crate::{
    models::message::{CreateMessageSchema, UpdateMessageSchema},
    models::database::Db,
};

pub async fn list_messages(Path(channel_id): Path<i64>, Extension(pool): Extension<PgPool>) -> Response {
    match Db::list_messages(channel_id, &pool).await {
        Ok(messages) => {
            let body = Json(messages);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn get_message(Path((channel_id, message_id)): Path<(i64, i64)>, Extension(pool): Extension<PgPool>) -> Response {
    match Db::get_message(channel_id, message_id, &pool).await {
        Ok(message) => {
            let body = Json(message);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn delete_message(Path((channel_id, message_id)): Path<(i64, i64)>, Extension(pool): Extension<PgPool>) -> Response {
    match Db::delete_message(channel_id, message_id, &pool).await {
        Ok(deleted) => {
            let body = Json(deleted);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn update_message(
    Path((channel_id, message_id)): Path<(i64, i64)>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<UpdateMessageSchema>
) -> Response {
    match Db::update_message(channel_id, message_id, &message.message, &pool).await {
        Ok(updated) => {
            let body = Json(updated);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn create_message(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(message): Json<CreateMessageSchema>
) -> Response {
    match Db::create_message(channel_id, &message.message, &message.user_id, &pool).await {
        Ok(created) => {
            let body = Json(created);
            (StatusCode::CREATED, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

