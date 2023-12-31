use axum::extract::Path;
use axum::http::StatusCode;
use axum::{Extension, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::db::Db;
use crate::errors::FlukeApiError;
use crate::models::channel::{ChannelModel, CreateChannelSchema, UpdateChannelSchema};

pub async fn list_channels(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<ChannelModel>>), FlukeApiError> {
    let channels = Db::list_channels(&pool)
        .await
        .map_err(FlukeApiError::from)?;
    Ok((StatusCode::OK, Json(channels)))
}

pub async fn get_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<ChannelModel>), FlukeApiError> {
    let channel = Db::get_channel(channel_id, &pool)
        .await
        .map_err(FlukeApiError::from)?;
    Ok((StatusCode::OK, Json(channel)))
}

pub async fn update_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<UpdateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), FlukeApiError> {
    if channel.name.is_empty() {
        return Err(FlukeApiError::BadRequest(
            "A channel name is required".to_string(),
        ));
    }
    let updated_channel = Db::update_channel(channel_id, &channel.name, &pool)
        .await
        .map_err(FlukeApiError::from)?;
    Ok((StatusCode::OK, Json(updated_channel)))
}

pub async fn create_channel(
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<CreateChannelSchema>,
) -> Result<(StatusCode, Json<ChannelModel>), FlukeApiError> {
    if channel.name.is_empty() {
        return Err(FlukeApiError::BadRequest(
            "A channel name is required".to_string(),
        ));
    }

    let created_channel = Db::create_channel(&channel.name, &pool)
        .await
        .map_err(FlukeApiError::from)?;
    Ok((StatusCode::CREATED, Json(created_channel)))
}

pub async fn delete_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), FlukeApiError> {
    let _ = Db::delete_channel(channel_id, &pool)
        .await
        .map_err(FlukeApiError::from)?;
    Ok((StatusCode::OK, Json(json!({"channel": "Channel deleted"}))))
}
