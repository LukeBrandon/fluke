use axum::extract::{Path,Json};
use axum::http::StatusCode;
use axum::Extension;
use serde_json::{Value, json};
use sqlx::PgPool;

use crate::errors::CustomError;
use crate::db::Db;
use crate::models::user::{CreateUserSchema, UpdateUserSchema, UserModel, LoginUserSchema, UserLoginResponse};

pub async fn list_users(Extension(pool): Extension<PgPool>) -> Result<(StatusCode, Json<Vec<UserModel>>), CustomError> {
    let users = Db::list_users(&pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(users)))
}

pub async fn create_user(Extension(pool): Extension<PgPool>, Json(user): Json<CreateUserSchema>) -> Result<(StatusCode, Json<UserModel>), CustomError> {
    let created_user = Db::create_user(user, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::CREATED, Json(created_user)))
}

pub async fn get_user(Path(user_id): Path<i64>, Extension(pool): Extension<PgPool>) -> Result<(StatusCode, Json<UserModel>), CustomError>  {
    let user = Db::get_user(user_id, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user(Path(user_id): Path<i64>, Extension(pool): Extension<PgPool>, Json(user): Json<UpdateUserSchema>) -> Result<(StatusCode, Json<UserModel>), CustomError> {
    let updated_user = Db::update_user(user_id, user, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(updated_user)))
}

pub async fn delete_user_soft(Path(user_id): Path<i64>, Extension(pool): Extension<PgPool>) -> Result<(StatusCode, Json<Value>), CustomError> {
    let _ = Db::delete_user_soft(user_id, &pool).await.map_err(CustomError::from)?;
    Ok((StatusCode::OK, Json(json!({"message": "User deleted", "user_id": user_id.to_string()}))))
}

pub async fn verify_user(Extension(pool): Extension<PgPool>, Json(credentials): Json<LoginUserSchema>) -> Result<(StatusCode, Json<UserLoginResponse>), CustomError> {
    let user_model = Db::verify_user(credentials.clone(), &pool).await.map_err(CustomError::from)?
        .ok_or_else(|| CustomError::NotFound(credentials.email.to_string()))?;

    let response = UserLoginResponse {
        status: "Logged in".to_string(),
        user_id: user_model.id,
    };
    Ok((StatusCode::OK, Json(response)))
}
