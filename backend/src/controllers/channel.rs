use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde_json::{json, Value};

use sqlx::PgPool;

use crate::errors::CustomError;

use crate::models::channel::{CreateChannelSchema, ChannelModel};
use crate::models::message::MessageModel;

pub async fn list_channels(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM channel";

    let task: Vec<ChannelModel> = sqlx::query_as::<_, ChannelModel>(sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}

pub async fn get_channel(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<ChannelModel>, CustomError> {
    let sql = "SELECT * FROM channel where id = ($1)";

    let channel: ChannelModel = sqlx::query_as::<_, ChannelModel>(sql)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::ChannelNotFound
        })?;

    Ok(Json(channel))
}

pub async fn update_channel(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<CreateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), CustomError> {
    let updated = sqlx::query_as!(
        ChannelModel,
        "UPDATE channel SET name=$1 WHERE id=$2 RETURNING *",
        &channel.name,
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

pub async fn create_channel(
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<CreateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), CustomError> {
    if channel.name.is_empty() {
        return Err(CustomError::BadRequest);
    }

    let created = sqlx::query_as!(
        ChannelModel,
        "INSERT INTO channel (name) VALUES ($1) RETURNING *",
        &channel.name
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        CustomError::InternalServerError
    })?;

    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn delete_channel(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "DELETE FROM channel WHERE id = ($1)";

    let _ = sqlx::query(sql)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::ChannelNotFound
        })?;

    Ok((StatusCode::OK, Json(json!({"channel": "Channel deleted"}))))
}

pub async fn load_channel_messages(
    Extension(pool): Extension<PgPool>,
    Path(channel_id): Path<i64>
) -> impl IntoResponse {

    let channel_messages: Vec<MessageModel> = sqlx::query_as!(
        MessageModel,
        "SELECT * FROM message WHERE channel_id = $1",
        &channel_id)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(channel_messages))
}
