use axum::{
    extract::{Json, Path},
    http::StatusCode,
    Extension,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::{
    errors::{CustomError, SignupError},
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

pub async fn signup_user(
    Extension(pool): Extension<PgPool>,
    Json(mut user): Json<CreateUserSchema>,
) -> Result<(StatusCode, Json<UserModel>), (StatusCode, String)> {
    user.email = user.email.to_lowercase();
    match create_user(user, pool).await {
        Ok(user_model) => Ok((StatusCode::CREATED, Json(user_model))),
        Err(e) => {
            let status = match e {
                SignupError::NonUniqueIdError => StatusCode::CONFLICT,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            };
            Err((status, e.to_string()))
        }
    }
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
            UPDATE fluke_user SET first_name=($2), last_name=($3), password=($4)
            WHERE id=$1
            RETURNING *
        "#,
        id,
        user.first_name,
        user.last_name,
        user.password
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(user_model)))
}

pub async fn new_user(
    Extension(pool): Extension<PgPool>,
    Json(user): Json<CreateUserSchema>,
) -> Result<(StatusCode, Json<UserModel>), CustomError> {
    let user_model: UserModel = create_user(user, pool)
        .await
        .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(user_model)))
}

pub async fn delete_user(
    Path(id): Path<i64>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Value>), CustomError> {
    // If user exists 404
    sqlx::query_as!(UserModel, "SELECT * FROM fluke_user WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::UserNotFound)?;

    let sql = "DELETE FROM fluke_user WHERE id = $1";

    let _ = sqlx::query(sql)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::UserNotFound)?;

    Ok((StatusCode::OK, Json(json!({"message": "User deleted"}))))
}

async fn create_user(user: CreateUserSchema, pool: PgPool) -> Result<UserModel, SignupError> {
    let user_model: UserModel = sqlx::query_as!(
        UserModel,
        r#"
        INSERT INTO fluke_user (first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        user.first_name,
        user.last_name,
        user.email.to_lowercase(),
        user.password
    )
    .fetch_one(&pool)
    .await
    .map_err(SignupError::from)?;

    Ok(user_model)
}
