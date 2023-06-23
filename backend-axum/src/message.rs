use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateMessageSchema {
    message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
struct MessageModel {
    id: i64,
    message: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

pub async fn list_messages(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let sql = "SELECT * FROM message ".to_string();

    let task = sqlx::query_as::<_, MessageModel>(&sql)
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(task))
}
