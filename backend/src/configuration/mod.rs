use crate::configuration::models::{FlukeConfig, FlukePort};
use dotenvy::dotenv;
use std::env;

pub mod models;

/// Loads the database url from .evn file
pub fn load_database_url() -> String {
    env::var("DATABASE_URL").expect("Mandatory DATABASE_URL was not provided")
}

/// Fetches the port number from env var PORT or None
pub fn load_port() -> Option<FlukePort> {
    let env_port: String = env::var("PORT").ok()?;
    let port = env_port.parse::<u16>().ok()?;
    Some(FlukePort::from(port))
}

/// Fetches the port number from env var PORT or None
pub fn load_hmac() -> String {
    env::var("HMAC_KEY").expect(
        "Mandatory HMAC_KEY was not provided, use 'openssl rand -base64 48' to generate one",
    )
}

/// Loads from a .env file the configuration for Fluke
pub fn load_config() -> FlukeConfig {
    dotenv().expect("No .env file provided.");

    FlukeConfig {
        database_url: load_database_url(),
        port: load_port().unwrap_or(FlukePort::default()),
        hmac_key: load_hmac(),
    }
}
