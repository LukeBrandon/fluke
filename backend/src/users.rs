use std::fmt;

use crate::FlukeDb;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{fairing::AdHoc, routes};
use rocket_db_pools::{sqlx, Connection};
use sqlx::FromRow;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

// Matches fields from user registration form
// In the future, hash password before storage
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateUserSchema {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

// Likely want to add 'Optional' fields for last name
// If Optional fields added, change .fetch_* to .fetch_optional(...)
#[derive(Debug, Clone, Deserialize, Serialize, FromRow, FromForm)]
pub struct UserModel {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum SignupError {
    NonUniqueIndexError,
    UnknownQueryError,
    UnknownDatabaseError,
}

impl From<sqlx::Error> for SignupError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::Database(db_error) => {
                let pg_error = db_error.downcast::<sqlx::postgres::PgDatabaseError>();
                match pg_error.code() {
                    "23505" => {
                        println!("Duplicate username key constraint violation detected.");
                        SignupError::NonUniqueIndexError
                    }
                    _ => {
                        println!("-- An error the server didn't account for --");
                        println!("{:?}", pg_error);
                        SignupError::UnknownQueryError
                    }
                }
            }
            _ => {
                println!("Something else happened");
                SignupError::UnknownDatabaseError
            }
        }
    }
}

impl std::fmt::Display for SignupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignupError::NonUniqueIndexError => {
                write!(f, "Duplicate username or email contained a duplicate key.")
            }
            SignupError::UnknownQueryError => {
                write!(f, "Database query contained an unspecified error.")
            }
            SignupError::UnknownDatabaseError => write!(f, "Database error, not query related."),
        }
    }
}

#[get("/users/<username>")]
pub async fn read_user_username(
    mut db: Connection<FlukeDb>,
    username: String,
) -> Result<Option<Json<UserModel>>> {
    let user = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&mut *db)
    .await?;

    Ok(user.map(Json))
}

#[get("/users")]
pub async fn list_users(mut db: Connection<FlukeDb>) -> Result<Json<Vec<UserModel>>> {
    let users = sqlx::query_as!(
        UserModel,
        "SELECT id, username, first_name, last_name, email, password FROM users"
    )
    .fetch_all(&mut *db)
    .await?;

    Ok(Json(users))
}

pub async fn create_user(
    mut db: Connection<FlukeDb>,
    user: CreateUserSchema,
) -> Result<UserModel, SignupError> {
    let query = sqlx::query_as!(
        UserModel,
        r#"
        INSERT INTO users (username, first_name, last_name, email, password)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#,
        user.username,
        user.first_name,
        user.last_name,
        user.email,
        user.password
    )
    .fetch_one(&mut *db)
    .await
    .map_err(SignupError::from)?;
    Ok(query)
}

#[delete("/users/<id>")]
pub async fn delete_user(mut db: Connection<FlukeDb>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&mut *db)
        .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
}

pub fn users_stage() -> AdHoc {
    AdHoc::on_ignite("Users Stage", |rocket| async {
        rocket.mount(
            "/users/",
            routes![read_user_username, list_users, delete_user],
        )
    })
}
