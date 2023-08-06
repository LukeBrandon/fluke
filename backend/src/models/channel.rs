use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct CreateChannelSchema {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct ChannelModel {
    pub id: i64,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
