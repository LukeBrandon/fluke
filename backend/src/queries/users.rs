use sqlx::PgPool;

use crate::errors::CustomError;
use crate::models::{
    database::Db,
    user::{
        UserModel,
        UserLoginResponse,
        LoginUserSchema,
        UpdateUserSchema,
        CreateUserSchema
    }
};

impl Db {
    pub async fn list_users(pool: &PgPool) -> Result<Vec<UserModel>, CustomError> {
        let result: Vec<UserModel> = sqlx::query_as!(
            UserModel,
            "SELECT * FROM fluke_user WHERE deleted=false"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(result)
    }

    pub async fn get_user(user_id: i64, pool: &PgPool) -> Result<UserModel, CustomError> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM fluke_user WHERE id = $1 AND deleted=false",
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(user)
    }

    pub async fn delete_user(user_id: i64, pool: &PgPool) -> Result<(), CustomError> {
        sqlx::query!(
            "UPDATE fluke_user SET deleted=true WHERE id=$1",
            user_id
        )
        .execute(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(())
    }

    pub async fn login_user(params: LoginUserSchema, pool: &PgPool) -> Result<UserLoginResponse, CustomError> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM fluke_user WHERE email = $1 AND password = $2",
            params.email.to_lowercase(),
            params.password
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        match user {
            Some(user_model) => {
                let response = UserLoginResponse {
                    status: "logged in".to_string(),
                    user_id: user_model.id,
                };
                Ok(response)
            }
            None => Err(CustomError::NotFound(params.email.to_string())),
        }
    }

    pub async fn update_user(user_id: i64, user: UpdateUserSchema, pool: &PgPool) -> Result<UserModel, CustomError> {
        let updated = sqlx::query_as!(
            UserModel,
            "UPDATE fluke_user SET first_name=$1, last_name=$2, password=$3 WHERE id=$4 RETURNING *",
            user.first_name,
            user.last_name,
            user.password,
            user_id
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::DatabaseError(e.to_string())
        })?;

        Ok(updated)
    }

    pub async fn create_user(user: CreateUserSchema, pool: &PgPool) -> Result<UserModel, CustomError> {
        let created = sqlx::query_as!(
            UserModel,
            "INSERT INTO fluke_user (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
            user.first_name,
            user.last_name,
            user.email.to_lowercase(),
            user.password
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database error: {}", e);
            CustomError::InternalServerError
        })?;

        Ok(created)
    }
}
