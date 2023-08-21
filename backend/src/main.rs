use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        request::Parts,
        HeaderValue, Method, StatusCode,
    },
    Router,
};
use tower::ServiceBuilder;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, time::Duration, sync::Arc};
use tower_http::{add_extension::AddExtensionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

use crate::configuration::models::FlukeConfiguration;

mod configuration;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<FlukeConfiguration>,
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    let config = configuration::load_config();
    let port = config.port.0;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // Initialize our logger
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    // Connect to our database
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Could not connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Error running migrations");

    let cors = CorsLayer::new()
        // sync with frontend for now
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // Set up middleware
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .layer(AddExtensionLayer::new(pool))
        .into_inner();

    // Build our server
    let app = Router::new()
        .merge(routes::user_router())
        .merge(routes::channel_router())
        .merge(routes::message_router())
        .layer(middleware_stack);

    // Run our service with hyper
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// we can also write a custom extractor that grabs a connection from the pool
// which setup is appropriate depends on your application
struct DatabaseConnection(sqlx::pool::PoolConnection<sqlx::Postgres>);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    PgPool: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = PgPool::from_ref(state);

        let conn = pool.acquire().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
