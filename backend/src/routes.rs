use axum::{routing::get, Router};

use crate::controllers;
use crate::ApiContext;

pub(crate) fn user_router() -> Router<ApiContext> {
    Router::new()
        .route(
            "/api/users",
            get(controllers::user::list_users).post(controllers::user::create_user),
        )
        .route("/api/users/login", get(controllers::user::login_user))
        .route(
            "/api/user",
            get(controllers::user::get_current_user)
                .put(controllers::user::update_user)
                .delete(controllers::user::delete_user_soft),
        )
}

pub fn message_router() -> Router<ApiContext> {
    let router = Router::new()
        .route(
            "/",
            get(controllers::message::list_messages).post(controllers::message::create_message),
        )
        .route(
            "/:message_id",
            get(controllers::message::get_message)
                .put(controllers::message::update_message)
                .delete(controllers::message::delete_message),
        );
    Router::new().nest("/channels/:channel_id/messages", router)
}

pub fn channel_router() -> Router<ApiContext> {
    let router = Router::new()
        .route(
            "/",
            get(controllers::channel::list_channels).post(controllers::channel::create_channel),
        )
        .route(
            "/:channel_id",
            get(controllers::channel::get_channel)
                .put(controllers::channel::update_channel)
                .delete(controllers::channel::delete_channel),
        );
    Router::new().nest("/api/channels", router)
}
