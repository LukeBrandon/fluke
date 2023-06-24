use dotenvy;
use std::env;
use crate::configuration::models::{FlukeConfiguration, FlukePort};

mod models;

/// Loads the database url from .evn file
fn load_database_url() -> String {
    env::var("DATABASE_URL").expect("Mandatory DATABASE_URL was not provided")
}

/// Fetches the port number from env var PORT or None
fn load_port() -> Option<FlukePort> {
    let env_port: String = env::var("PORT").ok()?;
    let port = env_port.parse::<u16>().ok()?;
    Some(FlukePort::from(port))
}

/// Loads from a .env file the configuration for Fluke
pub fn load_config() -> FlukeConfiguration {
    dotenvy::dotenv().expect("No .env file provided.");

    //TODO: use constuctor to load in teh values
    FlukeConfiguration {
        database_url: load_database_url(),
        port: load_port().unwrap_or(FlukePort::default()),
        ..FlukeConfiguration::default()
    }
}
