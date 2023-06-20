#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::fs::NamedFile;
use rocket::Request;
use rocket::{fairing, Build, Rocket};
use rocket_db_pools::{sqlx, Database};
use std::path::Path;

mod messages;
mod users;

#[derive(Database)]
#[database("fluke")]
pub struct FlukeDb(sqlx::PgPool);

#[options("/<_..>")]
fn all_options() {
    // Intentionally empty
}

#[catch(404)]
async fn not_found(_req: &Request<'_>) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join("404.html"))
        .await
        .ok()
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match FlukeDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
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
        .attach(FlukeDb::init())
        .attach(fairing::AdHoc::try_on_ignite(
            "SQLx Migrations",
            run_migrations,
        ))
        .mount("/", routes![all_options])
        .attach(messages::messages_stage())
        .attach(users::users_stage())
        .register("/", catchers![not_found])
}
