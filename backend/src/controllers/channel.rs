use axum::extract::Path;
use axum::http::StatusCode;
use serde_json::{json, Value};
use axum::{Extension, Json};
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::db::Db;
use crate::models::channel::{CreateChannelSchema, UpdateChannelSchema, ChannelModel};

pub async fn list_channels(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<ChannelModel>>, CustomError> {
    let channels = Db::list_channels(&pool).await.map_err(CustomError::from)?;
    Ok(Json(channels))
}

pub async fn get_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<ChannelModel>, CustomError> {
    let channel = Db::get_channel(channel_id, &pool).await.map_err(CustomError::from)?;
    Ok(Json(channel))
}

pub async fn update_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<UpdateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), CustomError> {
    let updated_channel = Db::update_channel(channel_id, &channel.name, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(updated_channel)))
}

pub async fn create_channel(
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<CreateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), CustomError> {
    let created_channel = Db::create_channel(&channel.name, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::CREATED, Json(created_channel)))
}

pub async fn delete_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let _ = Db::delete_channel(channel_id, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(json!({"channel": "Channel deleted"}))))
}

