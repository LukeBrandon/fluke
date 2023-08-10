use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::controllers;

pub fn setup_routes() -> Router<()> {

    let user_router = Router::new()
        .route("/users", get(controllers::user::list_users))
        .route("/users", post(controllers::user::new_user))
        .route("/users/signup", post(controllers::user::signup_user))
        .route("/users/login", get(controllers::user::login_user))
        .route("/users/:user_id", get(controllers::user::get_user))
        .route("/users/:user_id", put(controllers::user::update_user))
        .route("/users/:user_id", delete(controllers::user::delete_user));

    let message_router = Router::new()
        .route("/",
               get(controllers::message::list_messages)
               .post(controllers::message::create_message)
              )
        .route("/:message_id",
               get(controllers::message::get_message)
               .put(controllers::message::update_message)
               .delete(controllers::message::delete_message)
              );

    let channel_router = Router::new()
        .route("/",
               get(controllers::channel::list_channels)
               .post(controllers::channel::create_channel)
              )
        .route("/:channel_id",
               get(controllers::channel::get_channel)
               .put(controllers::channel::update_channel)
               .delete(controllers::channel::delete_channel)
              );

    let app = Router::new()
        .merge(user_router)
        .nest("/channels", channel_router)
        .nest("/channels/:channel_id/messages", message_router);

    return app
}
