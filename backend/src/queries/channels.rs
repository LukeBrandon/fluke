use sqlx::PgPool;
use crate::errors::CustomError;
use crate::models::{
    database::Db,
    channel::ChannelModel
};

impl Db {
    pub async fn list_channels(pool: &PgPool) -> Result<Vec<ChannelModel>, CustomError> {
        let channels = sqlx::query_as!(
            ChannelModel,
            "SELECT * FROM channel"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(channels)
    }

    pub async fn get_channel(channel_id: i64, pool: &PgPool) -> Result<ChannelModel, CustomError> {
        let channel = sqlx::query_as!(
            ChannelModel,
            "SELECT * FROM channel WHERE id = $1",
            channel_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(channel)
    }

    pub async fn update_channel(
        channel_id: i64,
        name: &str,
        pool: &PgPool
    ) -> Result<ChannelModel, CustomError> {
        let updated = sqlx::query_as!(
            ChannelModel,
            "UPDATE channel SET name = $1 WHERE id = $2 RETURNING *",
            name,
            channel_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(updated)
    }

    pub async fn create_channel(name: &str, pool: &PgPool) -> Result<ChannelModel, CustomError> {
        if name.is_empty() {
            return Err(CustomError::BadRequest);
        }

        let created = sqlx::query_as!(
            ChannelModel,
            "INSERT INTO channel (name) VALUES ($1) RETURNING *",
            name
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(created)
    }

    pub async fn delete_channel(channel_id: i64, pool: &PgPool) -> Result<(), CustomError> {
        sqlx::query!(
            "DELETE FROM channel WHERE id = $1",
            channel_id
        )
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }
}

