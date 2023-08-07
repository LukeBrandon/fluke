use configuration::{load_config, FlukeConfiguration};
use controllers;
use models;
use errors;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    routing::{delete, get, post, put},
    Router,
};

use tower::ServiceBuilder;

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::SocketAddr, time::Duration};
use axum::http::Request;

#[cfg(test)]
mod tests {
    use dotenvy;

    #[sqlx::test]
    async fn setup_database() {
        let config = load_config();
        let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.database_url)
            .await
            .expect("Could not connect to database");
}

    #[sqlx::test]
    async fn test_list_users() {
        // Setup: Connect to the test database, possibly insert mock data
        let mut conn = pool.acquire().await?;

        // Create a GET request for the list users endpoint
        let request = Request::builder()
            .method("GET")
            .uri("/users")
            .body(Body::empty())
            .unwrap();

        let response = app.handle(request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}


