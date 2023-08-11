use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use sqlx::PgPool;
use serde_json::json;

use crate::models::{
    database::Db,
    channel::{CreateChannelSchema, UpdateChannelSchema}
};

pub async fn list_channels(
    Extension(pool): Extension<PgPool>
) -> Response {
    match Db::list_channels(&pool).await {
        Ok(channels) => {
            let body = Json(channels);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn get_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Response {
    match Db::get_channel(channel_id, &pool).await {
        Ok(channel) => {
            let body = Json(channel);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn update_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<UpdateChannelSchema>,
) -> Response {
    match Db::update_channel(channel_id, &channel.name, &pool).await {
        Ok(updated) => {
            let body = Json(updated);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn create_channel(
    Extension(pool): Extension<PgPool>,
    Json(channel): Json<CreateChannelSchema>,
) -> Response {
    match Db::create_channel(&channel.name, &pool).await {
        Ok(created) => {
            let body = Json(created);
            (StatusCode::CREATED, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn delete_channel(
    Path(channel_id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Response {
    match Db::delete_channel(channel_id, &pool).await {
        Ok(_) => {
            let body = Json(json!({"channel": "Channel deleted"}));
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

