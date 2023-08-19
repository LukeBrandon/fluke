use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserModel {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
pub struct CreateUserSchema {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLoginResponse {
    pub status: String,
    pub user_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i64,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Channel {
    pub id: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageResponse {
    pub status: String,
    pub data: Message,
}
