use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserLoginResponse {
    pub status: String,
    pub user_id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateUserSchema {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}
// Likely want to add 'Optional' fields for last name
// If Optional fields added, change .fetch_* to .fetch_optional(...)
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
pub struct UserModel {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub password: String,
}

