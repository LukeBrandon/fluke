#[macro_use]
extern crate rocket;

use dotenv::dotenv;

use rocket::response::status::Created;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{fairing, fairing::AdHoc, Build, Rocket};

use rocket_db_pools::{sqlx, Connection, Database};

use sqlx::FromRow;

#[derive(Database)]
#[database("fluke")]
struct Fluke(sqlx::PgPool);

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
struct Message {
    message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
struct MessageWithId {
    id: i64,
    message: String,
}

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/<id>")]
async fn read_message(mut db: Connection<Fluke>, id: i64) -> Result<Json<MessageWithId>> {
    let message = sqlx::query_as!(MessageWithId, "SELECT * FROM message WHERE id = $1", id)
        .fetch_one(&mut *db)
        .await?;

    Ok(Json(message))
}

#[get("/")]
async fn list_messages(mut db: Connection<Fluke>) -> Result<Json<Vec<MessageWithId>>> {
    let query = sqlx::query_as!(MessageWithId, "SELECT id, message FROM message");
    let messages: Vec<MessageWithId> = query.fetch_all(&mut *db).await?;

    Ok(Json(messages))
}

#[post("/", data = "<message>")]
async fn create_message(
    mut db: Connection<Fluke>,
    message: Json<Message>,
) -> Result<Created<Json<Message>>> {
    sqlx::query_as!(
        Message,
        "INSERT INTO message (message) VALUES ($1)",
        &message.message
    )
    .execute(&mut *db)
    .await?;

    Ok(Created::new("/").body(message))
}

#[delete("/<id>")]
async fn delete_message(mut db: Connection<Fluke>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query_as!(
        Message,
        "DELETE FROM message WHERE id = ($1)",
        id
    )
    .execute(&mut *db)
    .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to Fluke!"
}

/// This attempts to run migrations from the `./migrations` directory
/// This was gotten from the rocket.rs sqlx example [here](https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/databases/src/sqlx.rs#L87)
async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Fluke::fetch(&rocket) {
        Some(db) => match sqlx::migrate!(".//migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

#[launch]
fn rocket() -> _ {
    dotenv()
        .ok()
        .expect("Could not load environment variables from .env");

    rocket::build()
        .attach(Fluke::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount("/", routes![index])
        .mount(
            "/messages/",
            routes![read_message, list_messages, create_message, delete_message],
        )
}
