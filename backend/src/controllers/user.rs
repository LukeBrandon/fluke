use axum::extract::{Path, Query, Json};
use axum::http::StatusCode;
use axum::Extension;
use axum::response::{IntoResponse, Response};
use sqlx::PgPool;

use crate::{
    models::user::{CreateUserSchema, UpdateUserSchema, LoginUserSchema},
    models::database::Db,
};

pub async fn list_users(Extension(pool): Extension<PgPool>) -> Response {
    match Db::list_users(&pool).await {
        Ok(users) => {
            let body = Json(users);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn login_user(Extension(pool): Extension<PgPool>, Query(params): Query<LoginUserSchema>) -> Response {
    match Db::login_user(params, &pool).await {
        Ok(response) => {
            let body = Json(response);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn get_user(Path(user_id): Path<i64>, Extension(pool): Extension<PgPool>) -> Response {
    match Db::get_user(user_id, &pool).await {
        Ok(user) => {
            let body = Json(user);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn signup_user(Extension(pool): Extension<PgPool>, Json(user): Json<CreateUserSchema>) -> Response {
    match Db::create_user(user, &pool).await {
        Ok(user_model) => {
            let body = Json(user_model);
            (StatusCode::CREATED, body).into_response()
        },
        Err(e) => e.into_response(),
    }
}

pub async fn update_user(Path(user_id): Path<i64>, Extension(pool): Extension<PgPool>, Json(user): Json<UpdateUserSchema>) -> Response {
    match Db::update_user(user_id, user, &pool).await {
        Ok(updated) => {
            let body = Json(updated);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn new_user(Extension(pool): Extension<PgPool>, Json(user): Json<CreateUserSchema>) -> Response {
    match Db::create_user(user, &pool).await {
        Ok(created) => {
            let body = Json(created);
            (StatusCode::CREATED, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}

pub async fn delete_user(
    Path(user_id): Path<i64>,
    Extension(pool): Extension<PgPool>
    ) -> Response {
    match Db::delete_user(user_id, &pool).await {
        Ok(deleted) => {
            let body = Json(deleted);
            (StatusCode::OK, body).into_response()
        }
        Err(e) => e.into_response(),
    }
}
