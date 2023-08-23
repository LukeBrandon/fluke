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

use crate::configuration::models::FlukeConfig;
use crate::errors::FlukeApiError;
use anyhow::Context;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

mod configuration;
mod controllers;
mod db;
mod errors;
mod models;
mod routes;

pub type Result<T, E = FlukeApiError> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<FlukeConfig>,
    pub pool: PgPool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = configuration::load_config();

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

    serve(config, pool).await?;
    Ok(())
}

fn api_router(api_context: ApiContext) -> Router {
    // Set up middleware
    let cors = CorsLayer::new()
        // sync with frontend for now
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .into_inner();

    Router::new()
        .merge(routes::user_router())
        .merge(routes::channel_router())
        .merge(routes::message_router())
        .layer(middleware_stack)
        .with_state(api_context)
}

pub async fn serve(config: FlukeConfig, pool: PgPool) -> anyhow::Result<()> {
    let api_context = ApiContext {
        config: Arc::new(configuration::load_config()),
        pool,
    };

    let app = api_router(api_context);
    let addr = SocketAddr::from(([127, 0, 0, 1], config.port.0));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
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
