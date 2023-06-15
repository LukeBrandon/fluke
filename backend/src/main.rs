#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use rocket::{fairing, fairing::AdHoc, Build, Rocket};
use rocket_db_pools::{sqlx, Database};

mod messages;
mod user;

#[derive(Database)]
#[database("fluke")]
pub struct FlukeDb(sqlx::PgPool);

#[get("/")]
fn index() -> &'static str {
    "Welcome to Fluke!"
}

/// This attempts to run migrations from the `./migrations` directory
/// This was gotten from the rocket.rs sqlx example [here](https://github.com/SergioBenitez/Rocket/blob/v0.5-rc/examples/databases/src/sqlx.rs#L87)
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
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .mount("/", routes![index])
        .attach(messages::messages_stage())
        .attach(user::users_stage())

}
