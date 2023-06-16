#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::{fairing, http, serde, Build, Request, Response, Rocket};
// rocket_cors arent used yet, but they are an alternative and could make things easier, leaving for now
use rocket::form::Form;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket_db_pools::{sqlx, Database};
use std::path::{Path};

mod messages;
mod user;

#[derive(Database)]
#[database("fluke")]
pub struct FlukeDb(sqlx::PgPool);

/// We need to create a fairing to to handle CORS restrictions
/// See https://docs.rs/rocket/latest/rocket/fairing/trait.Fairing.html
/// for information about what goes into the fairing and an example
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

/// This feels like the wrong way to handle this
#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new("../frontend/dist/index.html");
    NamedFile::open(path).await.ok()
}

/// Setter
#[post("/signup", data = "<data>")]
async fn signup(data: Form<user::UserModel>) {
    debug!("Received data: {:?}", data);
}

/// Catches all OPTION requests to get the CORS
#[options("/<_..>")]
fn all_options() {
    // Intentionally empty
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
        .attach(CORS)
        .mount("/", routes![index, all_options, signup])
        .mount("/", FileServer::from(relative!("../frontend/dist")))
        .attach(messages::messages_stage())
        .attach(user::users_stage())
}
