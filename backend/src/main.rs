#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::fs::NamedFile;
use rocket::{fairing, http, Build, Request, Response, Rocket};
use rocket_db_pools::{sqlx, Database};
use std::path::Path;
mod messages;
mod users;

pub struct CORS;
#[rocket::async_trait]
impl fairing::Fairing for CORS {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Add CORS headers to responses",
            kind: fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(http::Header::new(
            "Access-Control-Allow-Origin",
            request.headers().get_one("Origin").unwrap_or("*"),
        ));
        response.set_header(http::Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(http::Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(http::Header::new(
            "Access-Control-Allow-Credentials",
            "true",
        ));
    }
}

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
        .register("/", catchers![not_found])
        .mount("/", routes![all_options])
        .attach(messages::messages_stage())
        .attach(users::users_stage())
        .attach(CORS)
}
