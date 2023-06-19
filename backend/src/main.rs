#[macro_use]
extern crate rocket;
use dotenvy::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket::response::status::Created;
use rocket::serde::json::Json;
use rocket::{fairing, http, Build, Request, Response, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

use crate::users::{CreateUserSchema, SignupError, UserModel};

mod messages;
mod users;
type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("fluke")]
pub struct FlukeDb(sqlx::PgPool);

/// See https://docs.rs/rocket/latest/rocket/fairing/trait.Fairing.html
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

#[post("/signup", data = "<user>")]
async fn signup(
    db: Connection<FlukeDb>,
    user: Json<CreateUserSchema>,
) -> Result<Created<Json<UserModel>>, rocket::response::status::Custom<String>> {
    match users::create_user(db, user.into_inner()).await {
        Ok(user_model) => {
            let location = format!("/users/{}", user_model.id);
            Ok(Created::new(location).body(Json(user_model)))
        }
        Err(e) => {
            let status = match e {
                SignupError::NonUniqueIndexError => Status::Conflict,
                _ => Status::InternalServerError,
            };
            Err(rocket::response::status::Custom(status, e.to_string()))
        }
    }
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
        .mount("/", routes![all_options, signup])
        .mount("/", FileServer::from(relative!("static")))
        .attach(messages::messages_stage())
        .attach(users::users_stage())
}
