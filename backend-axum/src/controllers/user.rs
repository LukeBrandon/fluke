use axum::{
    extract::{Json, Path},
    http::StatusCode,
    Extension,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    errors::CustomError,
    models::user::{CreateUserSchema, UpdateUserSchema, UserModel},
};

pub async fn list_users(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<UserModel>>, CustomError> {
    let list_of_users: Vec<UserModel> = sqlx::query_as!(UserModel, "SELECT * FROM fluke_user")
        .fetch_all(&pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok(Json(list_of_users))
}

pub async fn get_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<UserModel>, CustomError> {
    let user: UserModel = sqlx::query_as!(UserModel, "SELECT * FROM fluke_user WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::UserNotFound)?;

    Ok(Json(user))
}

pub async fn update_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
    Json(user): Json<UpdateUserSchema>,
) -> Result<(StatusCode, Json<UserModel>), CustomError> {
    let user_model: UserModel = sqlx::query_as!(
        UserModel,
        r#"
            UPDATE fluke_user SET username=($2), first_name=($3), last_name=($4), password=($5)
            WHERE id=$1
            RETURNING *
        "#,
        id,
        user.username,
        user.first_name,
        user.last_name,
        user.password
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(user_model)))
}

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<CreateUserSchema>,
) -> Result<(StatusCode, Json<UserModel>), CustomError> {
    let user_model: UserModel = sqlx::query_as!(
        UserModel,
        r#"
        INSERT INTO fluke_user (username, first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        user.username,
        user.first_name,
        user.last_name,
        user.email,
        user.password
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(user_model)))
}

pub async fn delete_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    let sql = "DELETE FROM fluke_user WHERE id = $1";

    let _ = sqlx::query(&sql)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::UserNotFound)?;

    Ok((StatusCode::OK, Json(json!({"message": "User deleted"}))))
}
