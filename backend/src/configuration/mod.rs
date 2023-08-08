use crate::configuration::models::{FlukeConfiguration, FlukePort};
use dotenvy::dotenv;
use std::env;

mod models;

fn load_database_url() -> String {
    env::var("DATABASE_URL").expect("Mandatory DATABASE_URL was not provided")
}

fn load_port() -> Option<FlukePort> {
    let env_port: String = env::var("PORT").ok()?;
    let port = env_port.parse::<u16>().ok()?;
    Some(FlukePort::from(port))
}

pub fn load_config() -> FlukeConfiguration {
    dotenv().expect("No .env file provided.");

    FlukeConfiguration {
        database_url: load_database_url(),
        port: load_port().unwrap_or(FlukePort::default()),
    }
}
