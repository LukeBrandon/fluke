use sqlx::postgres::PgQueryResult;
use sqlx::PgPool;

use crate::db::Db;
use crate::models::user::{CreateUserSchema, LoginUserSchema, UpdateUserSchema, UserModel};

impl Db {
    pub async fn list_users(pool: &PgPool) -> Result<Vec<UserModel>, sqlx::Error> {
        sqlx::query_as!(UserModel, "SELECT * FROM fluke_user WHERE deleted=false")
            .fetch_all(pool)
            .await
    }

    pub async fn get_user(user_id: i64, pool: &PgPool) -> Result<UserModel, sqlx::Error> {
        sqlx::query_as!(
            UserModel,
            "SELECT * FROM fluke_user WHERE id = $1 AND deleted=false",
            user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn delete_user_soft(
        user_id: i64,
        pool: &PgPool,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("UPDATE fluke_user SET deleted=true WHERE id=$1", user_id)
            .execute(pool)
            .await
    }

    pub async fn verify_user(
        credentials: LoginUserSchema,
        pool: &PgPool,
    ) -> Result<Option<UserModel>, sqlx::Error> {
        sqlx::query_as!(
            UserModel,
            "SELECT * FROM fluke_user WHERE email = $1 AND password = $2",
            credentials.email.to_lowercase(),
            credentials.password
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn update_user(
        user_id: i64,
        user: UpdateUserSchema,
        pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
        sqlx::query_as!(
            UserModel,
            "UPDATE fluke_user SET first_name=$1, last_name=$2, password=$3 WHERE id=$4 RETURNING *",
            user.first_name,
            user.last_name,
            user.password,
            user_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn create_user(
        user: CreateUserSchema,
        pool: &PgPool,
    ) -> Result<UserModel, sqlx::Error> {
        sqlx::query_as!(
            UserModel,
            "INSERT INTO fluke_user (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
            user.first_name,
            user.last_name,
            user.email.to_lowercase(),
            user.password
        )
        .fetch_one(pool)
        .await
    }
}
