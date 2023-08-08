#[cfg(test)]
mod tests {
    use crate::init_app;
    use crate::init_db;
    use crate::models;
    use reqwest;
    use serde_json::json;
    use std::net::SocketAddr;
    use tracing_subscriber::EnvFilter;

    #[sqlx::test]
    async fn test_create_user() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .pretty()
            .init();

        let port = 8880;
        println!("Port: {}", port);
        let pool = init_db().await;
        let app = init_app(pool.clone());
        let addr = SocketAddr::from(([127, 0, 0, 1], port));

        let server = axum::Server::bind(&addr).serve(app.into_make_service());
        tracing::info!("Listening on {}", addr);

        // without this method, tests run forever...and ever
        // run server concurrently, so it doesn't block
        // and give the server a little time to start up
        let server_handle = tokio::spawn(server);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        let test_user = json!({
            "first_name": "Test",
            "last_name": "User",
            "email": "test@example.com",
            "password": "test_password"
        });

        let client = reqwest::Client::new();
        let resp = client
            .post(format!("http://{}{}", addr, "/users"))
            .json(&test_user)
            .send()
            .await
            .expect("Failed to send request");

        println!("Response status: {:?}", resp.status());

        let created_user: models::user::UserModel =
            resp.json().await.expect("Failed to parse response");

        assert_eq!(created_user.first_name, "Test");
        assert_eq!(created_user.last_name, "User");

        drop(server_handle);
    }
}
