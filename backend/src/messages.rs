extern crate rocket;
use crate::FlukeDb;
use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{fairing::AdHoc, routes};

use rocket_db_pools::{sqlx, Connection};

use sqlx::FromRow;

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateMessageSchema {
    message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
struct MessageModel {
    id: i64,
    message: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[get("/<id>")]
async fn read_message(mut db: Connection<FlukeDb>, id: i64) -> Result<Json<MessageModel>> {
    let message: MessageModel =
        sqlx::query_as!(MessageModel, "SELECT * FROM message WHERE id = $1", id)
            .fetch_one(&mut *db)
            .await?;

    Ok(Json(message))
}

#[get("/")]
async fn list_messages(mut db: Connection<FlukeDb>) -> Result<Json<Vec<MessageModel>>> {
    let messages = sqlx::query_as!(MessageModel, "SELECT * FROM message")
        .fetch_all(&mut *db)
        .await?;

    Ok(Json(messages))
}

#[post("/", data = "<message>")]
async fn create_message(
    mut db: Connection<FlukeDb>,
    message: Json<CreateMessageSchema>,
) -> Result<Created<Json<CreateMessageSchema>>> {
    sqlx::query_as!(
        MessageModel,
        "INSERT INTO message (message) VALUES ($1)",
        &message.message
    )
    .execute(&mut *db)
    .await?;

    Ok(Created::new("/").body(message))
}

#[delete("/<id>")]
async fn delete_message(mut db: Connection<FlukeDb>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query_as!(MessageModel, "DELETE FROM message WHERE id = ($1)", id)
        .execute(&mut *db)
        .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
}

pub fn messages_stage() -> AdHoc {
    AdHoc::on_ignite("Mesasges Stage", |rocket| async {
        rocket.mount(
            "/messages/",
            routes![read_message, list_messages, create_message, delete_message],
        )
    })
}
