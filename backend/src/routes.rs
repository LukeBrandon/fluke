use axum::{
    routing::get,
    Router,
};

use crate::controllers;


pub fn user_router() -> Router {
    Router::new()
        .route("/users",
            get(controllers::user::list_users)
            .post(controllers::user::create_user)
        )
        .route("/users/login", get(controllers::user::verify_user))
        .route("/users/:user_id",
            get(controllers::user::get_user)
            .put(controllers::user::update_user)
            .delete(controllers::user::delete_user_soft)
        )
}

pub fn message_router() -> Router {
    let router = Router::new()
    .route("/",
           get(controllers::message::list_messages)
           .post(controllers::message::create_message)
          )
    .route("/:message_id",
           get(controllers::message::get_message)
           .put(controllers::message::update_message)
           .delete(controllers::message::delete_message)
          );
    Router::new()
        .nest("/channels/:channel_id/messages", router)
}

pub fn channel_router() -> Router {
    let router = Router::new()
    .route("/",
           get(controllers::channel::list_channels)
           .post(controllers::channel::create_channel)
          )
    .route("/:channel_id",
           get(controllers::channel::get_channel)
           .put(controllers::channel::update_channel)
           .delete(controllers::channel::delete_channel)
          );
    Router::new()
        .nest("/channels", router)

}


