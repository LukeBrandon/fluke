#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::{fairing, http, serde, Request, Response, Build, Rocket};
// rocket_cors arent used yet, but they are an alternative and could make things easier, leaving for now
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_db_pools::{sqlx, Database};

mod messages;
mod user;

#[derive(Database)]
#[database("fluke")]
pub struct FlukeDb(sqlx::PgPool);

// We need to create a fairing to to handle CORS restrictions
// See https://docs.rs/rocket/latest/rocket/fairing/trait.Fairing.html
// for information about what goes into the fairing and an example
pub struct CORS;
#[rocket::async_trait]
impl fairing::Fairing for CORS {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Add CORS headers to responses",
            kind: fairing::Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        if response.status() != http::Status::NotFound {
            return
        }
        response.set_header(http::Header::new("Access-Control-Allow-Origin", "*"));
        // I dont think we need all of these, can cut some out later
        response.set_header(http::Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(http::Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(http::Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

// Getter
#[get("/")]
fn index() -> &'static str {
    "Welcome to Fluke!"
}

/// Setter
#[post("/", data = "<data>")]
async fn insert(data: serde::json::Json<Vec<String>>) {
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
        .attach(fairing::AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount("/", routes![index, all_options, insert])
        .attach(CORS)
        .attach(messages::messages_stage())
        .attach(user::users_stage())
}
