use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::{
    database::Db,
    message::MessageModel
};

impl Db {
    pub async fn list_messages(channel_id: i64, pool: &PgPool) -> Result<Vec<MessageModel>, CustomError> {
        let result: Vec<MessageModel> = sqlx::query_as!(
            MessageModel,
            "SELECT * FROM message where channel_id = $1",
            channel_id
            )
            .fetch_all(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                CustomError::DatabaseError(e.to_string())
            })?;

        Ok(result)
    }

    pub async fn get_message(channel_id: i64, message_id: i64, pool: &PgPool) -> Result<MessageModel, CustomError> {
        let message = sqlx::query_as!(
            MessageModel,
            "SELECT * FROM message where channel_id = $1 and id = $2",
            channel_id,
            message_id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                CustomError::DatabaseError(e.to_string())
            })?;

        Ok(message)
    }

    pub async fn delete_message(channel_id: i64, message_id: i64, pool: &PgPool) -> Result<(), CustomError> {
        sqlx::query!(
            "DELETE FROM message WHERE channel_id = $1 and id = $2",
            channel_id,
            message_id
            )
            .execute(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                CustomError::DatabaseError(e.to_string())
            })?;

        Ok(())
    }

    pub async fn update_message(
        channel_id: i64,
        message_id: i64,
        message: &str,
        pool: &PgPool
    ) -> Result<MessageModel, CustomError> {
        let updated = sqlx::query_as!(
            MessageModel,
            "UPDATE message SET message = $1 WHERE channel_id = $2 and id = $3 RETURNING *",
            &message,
            channel_id,
            message_id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                CustomError::DatabaseError(e.to_string())
            })?;

        Ok(updated)
    }

    pub async fn create_message(
        channel_id: i64,
        message: &str,
        user_id: &i64,
        pool: &PgPool
    ) -> Result<MessageModel, CustomError> {
        let created = sqlx::query_as!(
            MessageModel,
            "INSERT INTO message (channel_id, message, user_id) VALUES ($1, $2, $3) RETURNING *",
            channel_id,
            message,
            user_id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| {
                eprintln!("Database error: {}", e);
                CustomError::DatabaseError(e.to_string())
            })?;

        Ok(created)
    }
}
