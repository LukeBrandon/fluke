use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::{delete, get, post, put},
    Router,
};
use tower::ServiceBuilder;

use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
    add_extension::AddExtensionLayer,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, time::Duration};
use tracing_subscriber::EnvFilter;

mod configuration;
mod controllers;
mod errors;
mod models;
mod middleware;

#[tokio::main]
async fn main() {

    // RUST_LOG=backend_axum=info
    // either in .env or before cargo run
    dotenvy::dotenv().ok();

    let config = configuration::load_config();
    let port = config.port.0;

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.database_url)
        .await
        .expect("Could not connect to database");

    // Run migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Error running migrations");

    // create the application state
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::very_permissive()) // !! todo: change this bad
        .layer(AddExtensionLayer::new(pool))
        .into_inner();

    // build our application with some routes
    let app = Router::new()
        .route("/messages", get(controllers::message::list_messages))
        .route("/messages", post(controllers::message::create_message))
        .route("/messages/:id", put(controllers::message::update_message))
        .route("/messages/:id", get(controllers::message::get_message))
        .route(
            "/messages/:id",
            delete(controllers::message::delete_message),
        )
        .route("/users", get(controllers::user::list_users))
        .route("/users", post(controllers::user::new_user))
        .route("/users/signup", post(controllers::user::signup_user))
        .route("/users/login", post(controllers::user::login_user))
        .route("/users/:id", get(controllers::user::get_user))
        .route("/users/:id", put(controllers::user::update_user))
        .route("/users/:id", delete(controllers::user::delete_user))
        .layer(middleware_stack);

    // run it with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
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
