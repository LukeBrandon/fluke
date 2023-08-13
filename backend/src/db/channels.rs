use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

use crate::db::Db;
use crate::models::channel::ChannelModel;

impl Db {
    pub async fn list_channels(pool: &PgPool) -> Result<Vec<ChannelModel>, sqlx::Error> {
        sqlx::query_as!(ChannelModel, "SELECT * FROM channel")
            .fetch_all(pool)
            .await
    }

    pub async fn get_channel(channel_id: i64, pool: &PgPool) -> Result<ChannelModel, sqlx::Error> {
        sqlx::query_as!(
            ChannelModel,
            "SELECT * FROM channel WHERE id = $1",
            channel_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_channel(
        channel_id: i64,
        name: &str,
        pool: &PgPool,
    ) -> Result<ChannelModel, sqlx::Error> {
        sqlx::query_as!(
            ChannelModel,
            "UPDATE channel SET name = $1 WHERE id = $2 RETURNING *",
            name,
            channel_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn create_channel(name: &str, pool: &PgPool) -> Result<ChannelModel, sqlx::Error> {
        sqlx::query_as!(
            ChannelModel,
            "INSERT INTO channel (name) VALUES ($1) RETURNING *",
            name
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_channel(
        channel_id: i64,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM channel WHERE id = $1", channel_id)
            .execute(pool)
            .await
    }
}
