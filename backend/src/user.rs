use crate::FlukeDb;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{fairing::AdHoc, routes};
use password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;

use rocket_db_pools::{sqlx, Connection};

use sqlx::FromRow;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// Matches fields from user registration form
// In the future, hash password before storage 
#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateUserSchema {
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

// A row in the 'users' db table
// Likely want to add 'Optional' fields for last name
// If Optional fields added, change .fetch_* to .fetch_optional(...)
#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
struct UserModel {
    id: i64,
    username: String,
    first_name: String,
    last_name: String,
    email: String,
    // I dont think we want password here
    password: String,
}

#[get("/users/<id>")]
async fn read_user(mut db: Connection<FlukeDb>, id: i64) -> Result<Json<UserModel>> {
    let user = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE id = $1",
        id
    )
    .fetch_one(&mut *db)
    .await?;

    Ok(Json(user))
}

#[get("/users")]
async fn list_users(mut db: Connection<FlukeDb>) -> Result<Json<Vec<UserModel>>> {
    let users = sqlx::query_as!(
        UserModel,
        "SELECT id, username, first_name, last_name, email, password FROM users"
    )
    .fetch_all(&mut *db)
    .await?;

    Ok(Json(users))
}

#[post("/users", data = "<user>")]
async fn create_user(
    mut db: Connection<FlukeDb>,
    user: Json<CreateUserSchema>,
) -> Result<Created<Json<CreateUserSchema>>> {
    sqlx::query!(
        "INSERT INTO users (username, first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4, $5)",
        &user.username,
        &user.first_name,
        &user.last_name,
        &user.email,
        &user.password,
    )
    .execute(&mut *db)
    .await?;

    Ok(Created::new("/users").body(user))
}

#[delete("/users/<id>")]
async fn delete_user(mut db: Connection<FlukeDb>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
    .execute(&mut *db)
    .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
}

pub fn users_stage() -> AdHoc {
    AdHoc::on_ignite("Users Stage", |rocket| async {
        rocket.mount(
            "/",
            routes![read_user, list_users, create_user, delete_user],
        )
    })
}
