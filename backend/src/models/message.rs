use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateMessageSchema {
    pub message: String,
    pub user_id: i64,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MessageModel {
    pub id: i64,
    pub message: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub user_id: i64,
    pub channel_id: i64,
}
